use std::vec;

use semver::{Version, VersionReq};

mod dfs;
use dfs::Graph;

#[derive(Debug)]
struct Module {
    name: String,
    version: Version,
    requirements: Vec<Requirement>,
}

#[derive(Debug, Clone)]
struct Requirement {
    module: String,
    constraint: VersionReq,
}

fn solve_dependencies(modules: Vec<&Module>, top_module: Module) -> Result<Vec<(String, Version)>, String> {
    let mut graph = Graph::new();
    graph.loads_modules(modules);
    graph.sort_children();

    #[cfg(debug_assertions)]
    for (name, versions) in graph.vertex.iter() {
        for (version, vertice) in versions.iter() {
            println!("{}-{}: {:?}", name, version, vertice);
        }
    }
    Ok(Vec::new())
}

fn main() {
    let mut modules: Vec<&Module> = Vec::new();

    let module = Module {
        name: "A".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![Requirement {
            module: "B".to_string(),
            constraint: VersionReq::parse("^0.1.2").unwrap(),
        }],
    };
 
    modules.push(&module);

    let module = Module {
        name: "B".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![Requirement {
            module: "C".to_string(),
            constraint: VersionReq::parse("^1.1.2").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "B".to_string(),
        version: Version::parse("0.1.6").unwrap(),
        requirements: vec![Requirement {
            module: "C".to_string(),
            constraint: VersionReq::parse("^1.1.7").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "B".to_string(),
        version: Version::parse("0.2.0").unwrap(),
        requirements: vec![Requirement {
            module: "C".to_string(),
            constraint: VersionReq::parse("^1.1.7").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "C".to_string(),
        version: Version::parse("1.1.0").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    let module = Module {
        name: "C".to_string(),
        version: Version::parse("1.1.6").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    let module = Module {
        name: "C".to_string(),
        version: Version::parse("1.2.0").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    let top_module = Module {
        name: "A".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![Requirement {
            module: "B".to_string(),
            constraint: VersionReq::parse("^0.1.0").unwrap(),
        }],
    };

    let result = match solve_dependencies(modules, top_module) {
        Ok(result) => result,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    for (module, version) in result.iter() {
        println!("Using: {}: {}", module, version);
    }
}