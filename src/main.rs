use std::vec;

use semver::{Version, VersionReq};


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

fn solve_dependencies(modules: Vec<Module>, top_module: Module) -> Result<Vec<(String, Version)>, String> {
    Ok(Vec::new())
}

fn main() {
    let modules = vec![
        Module {
            name: "A".to_string(),
            version: Version::parse("0.1.0").unwrap(),
            requirements: vec![Requirement {
                module: "B".to_string(),
                constraint: VersionReq::parse("^0.1.2").unwrap(),
            }],
        },
        Module {
            name: "B".to_string(),
            version: Version::parse("0.1.0").unwrap(),
            requirements: vec![Requirement {
                module: "C".to_string(),
                constraint: VersionReq::parse("^1.1.2").unwrap(),
            }],
        },
        Module {
            name: "B".to_string(),
            version: Version::parse("0.1.6").unwrap(),
            requirements: vec![Requirement {
                module: "C".to_string(),
                constraint: VersionReq::parse("^1.1.7").unwrap(),
            }],
        },
        Module {
            name: "B".to_string(),
            version: Version::parse("0.2.0").unwrap(),
            requirements: vec![Requirement {
                module: "C".to_string(),
                constraint: VersionReq::parse("^1.1.7").unwrap(),
            }],
        },
        Module {
            name: "C".to_string(),
            version: Version::parse("1.1.0").unwrap(),
            requirements: vec![],
        },
        Module {
            name: "C".to_string(),
            version: Version::parse("1.1.6").unwrap(),
            requirements: vec![],
        },
        Module {
            name: "C".to_string(),
            version: Version::parse("1.2.0").unwrap(),
            requirements: vec![],
        },
    ];

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