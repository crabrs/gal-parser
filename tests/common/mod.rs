#![allow(unused_macros, dead_code)]

use std::{
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
};

macro_rules! parse_assert {
    ($rule:expr,$input:expr $(,)?) => {
        let span = <gql_parser::GQLParser as pest::Parser<_>>::parse($rule, $input)
            .unwrap()
            .map(|p| p.as_span().as_str())
            .fold(String::new(), |mut whole, s| {
                whole.push_str(s);
                whole
            });
        assert_eq!(span, $input.as_ref());
    };
}


macro_rules! test_queries {
    ($class:literal) => {
        for query in QueryResource::new("queries")
            .class($class)
            .queries()
            .unwrap()
            .map(|q| q.unwrap())
        {
            let gql = query.to_trimed_gql().unwrap();
            if !gql.starts_with("--ignore") {
                GQLParser::parse(Rule::gql_request, &gql).expect(&gql);
            }
        }
    };
}


pub(crate) struct QueryResource {
    path: PathBuf,
}

impl QueryResource {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub(crate) fn class(&self, class: &str) -> QueryClass {
        QueryClass {
            path: self.path.join(class),
        }
    }
}

pub(crate) struct QueryClass {
    path: PathBuf,
}

impl QueryClass {
    pub(crate) fn queries(&self) -> io::Result<Queries> {
        Ok(Queries {
            files: fs::read_dir(&self.path)?,
        })
    }
}

pub(crate) struct Queries {
    files: ReadDir,
}

impl Iterator for Queries {
    type Item = io::Result<Query>;

    fn next(&mut self) -> Option<Self::Item> {
        self.files
            .next()
            .map(|f| f.map(|f| Query { path: f.path() }))
    }
}

pub(crate) struct Query {
    path: PathBuf,
}

impl Query {
    pub(crate) fn to_trimed_gql(&self) -> io::Result<String> {
        fs::read_to_string(&self.path).map(|s| s.trim().to_string())
    }

    // pub(crate) fn to_gql(&self) -> io::Result<String> {
    //     fs::read_to_string(&self.path)
    // }

    // pub(crate) fn panic_parse(&self, rule: Rule) {
    //     let gql = self.to_trimed_gql().unwrap();
    //     match GQLParser::parse(rule, &gql) {
    //         Ok(_) => {}
    //         Err(e) => {
    //             panic!(
    //                 "{file}\n{split:#^width$}\n{query}\n{split:#^width$}\n{e:?}",
    //                 file = self.path.display(),
    //                 query = gql,
    //                 split = "#",
    //                 width = 120
    //             );
    //         }
    //     }
    // }
}
