use enum_as_inner::EnumAsInner;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    error::Error,
    rc::Rc,
};

use crate::utils;

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let line = utils::parse_line()?;

    let (_, commands) = parse_commands(&line).unwrap();
    let root = resolve_file_system(commands);

    let mut folder_sizes = vec![];
    root.borrow().traverse(&mut |dir| {
        let size = dir.total_size();
        if size <= 100000 {
            folder_sizes.push(size);
        }
    });

    Ok(folder_sizes.iter().sum::<usize>())
}

pub fn solution2() -> Result<usize, Box<dyn Error>> {
    let line = utils::parse_line()?;

    let (_, commands) = parse_commands(&line).unwrap();
    let root = resolve_file_system(commands);

    let mut deletion_candidates = BinaryHeap::<Reverse<usize>>::new();
    let remaining_space = 70_000_000 - root.borrow().total_size();
    let min_needed_space = 30_000_000usize.saturating_sub(remaining_space);

    root.borrow().traverse(&mut |dir| {
        let size = dir.total_size();
        if size >= min_needed_space {
            deletion_candidates.push(Reverse(size));
        }
    });

    Ok(deletion_candidates.pop().unwrap().0)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let cd = map(
        delimited(tag("$ cd "), parse_cd_target, opt(tag("\n"))),
        Command::ChangeDir,
    );

    let ls = map(
        preceded(tag("$ ls\n"), many0(alt((parse_file, parse_dir)))),
        Command::List,
    );

    many1(alt((cd, ls)))(input)
}

fn parse_cd_target(input: &str) -> IResult<&str, CdTarget> {
    alt((
        map(tag(".."), |_| CdTarget::Parent),
        map(parse_item_name, |s: &str| CdTarget::Dir(s.into())),
    ))(input)
}

fn parse_item_name(input: &str) -> IResult<&str, &str> {
    take_while(|s: char| s.is_alphanumeric() || s == '.' || s == '/')(input)
}

fn parse_file(input: &str) -> IResult<&str, Item> {
    map(
        pair(
            map_res(digit1, |s: &str| s.parse::<usize>()),
            delimited(space1, parse_item_name, opt(newline)),
        ),
        |(size, name)| {
            Item::File(File {
                size,
                name: name.into(),
            })
        },
    )(input)
}

fn parse_dir(input: &str) -> IResult<&str, Item> {
    map(
        delimited(tag("dir "), parse_item_name, opt(newline)),
        |name| Item::Dir(Dir::from(name)),
    )(input)
}

fn resolve_file_system(commands: impl IntoIterator<Item = Command>) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::from("/")));
    let mut pwd = Rc::clone(&root);

    for command in commands.into_iter().skip(1) {
        match command {
            Command::ChangeDir(CdTarget::Parent) => {
                let parent = pwd.borrow().parent.clone().unwrap();
                pwd = parent;
            }
            Command::ChangeDir(CdTarget::Dir(dirname)) => {
                let child = pwd.borrow().sub_dirs.get(&dirname).unwrap().clone();
                child.borrow_mut().parent = Some(pwd.clone());
                pwd = child;
            }
            Command::List(output) if pwd.borrow().is_empty() => {
                for item in output {
                    match item {
                        Item::Dir(dir) => {
                            pwd.borrow_mut()
                                .sub_dirs
                                .insert(dir.name.clone(), Rc::new(RefCell::new(dir)));
                        }
                        Item::File(file) => {
                            pwd.borrow_mut().files.insert(file.name.clone(), file);
                        }
                    }
                }
            }
            _ => {}
        };
    }

    root
}

#[derive(Eq, PartialEq, Debug)]
enum Command {
    ChangeDir(CdTarget),
    List(Vec<Item>),
}

#[derive(Eq, PartialEq, Debug)]
enum CdTarget {
    Parent,
    Dir(String),
}

#[derive(Eq, PartialEq, Debug, EnumAsInner)]
enum Item {
    File(File),
    Dir(Dir),
}

#[derive(Eq, PartialEq, Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn total_size(&self) -> usize {
        self.size
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Dir {
    name: String,
    files: HashMap<String, File>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn from(s: impl AsRef<str>) -> Self {
        Dir {
            name: s.as_ref().into(),
            files: Default::default(),
            sub_dirs: Default::default(),
            parent: Default::default(),
        }
    }

    fn total_size(&self) -> usize {
        self.files.values().map(|f| f.total_size()).sum::<usize>()
            + self
                .sub_dirs
                .values()
                .map(|d| d.borrow().total_size())
                .sum::<usize>()
    }

    fn is_empty(&self) -> bool {
        self.files.is_empty() && self.sub_dirs.is_empty()
    }

    fn traverse(&self, f: &mut impl FnMut(&Dir)) {
        f(self);
        for sub_dir in self.sub_dirs.values() {
            sub_dir.borrow().traverse(f);
        }
    }
}
