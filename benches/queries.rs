use criterion::{criterion_group, criterion_main, Criterion};
use gql_parser::{GQLParser, Rule};
use pest::Parser;

pub fn simple_read(c: &mut Criterion) {
    c.bench_function("simple read 1", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_1.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("simple read 2", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_2.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("simple read 3", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_3.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("simple read 4", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_4.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("simple read 5", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_5.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("simple read 6", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/finbench/simple_read_6.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
}

pub fn short_read(c: &mut Criterion) {
    c.bench_function("short read 1", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_1.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 2", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_2.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 3", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_3.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 4", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_4.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 5", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_5.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 6", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_6.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("short read 7", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/short_read_7.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
}


pub fn complex_read(c: &mut Criterion) {
    c.bench_function("complex read 1", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_1.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 2", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_2.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 3", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_3.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 4", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_4.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 5", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_5.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 6", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_6.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 7", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_7.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 8", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_8.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 9", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_9.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 10", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_10.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 11", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_11.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 12", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_12.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 13", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_13.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
    c.bench_function("complex read 14", |b| b.iter(|| {
        let gql = include_str!("../queries/snb/complex_read_14.gql");
        let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
    }));
}


pub fn insert(c: &mut Criterion) {
    c.bench_function("insert 1", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_1.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 2", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_2.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 3", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_3.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 4", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_4.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 5", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_5.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 6", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_6.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 7", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_7.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("insert 8", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/insert_8.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
}



pub fn delete(c: &mut Criterion) {
    c.bench_function("delete 1", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_1.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 2", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_2.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 3", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_3.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 4", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_4.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 5", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_5.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 6", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_6.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 7", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_7.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
    c.bench_function("delete 8", |b| {
        b.iter(|| {
            let gql = include_str!("../queries/snb/delete_8.gql");
            let _ = GQLParser::parse(Rule::gql_request, gql).unwrap();
        })
    });
}



criterion_group!(finbench, simple_read);
criterion_group!(snb, short_read, complex_read, insert, delete);
criterion_main!(finbench, snb);
