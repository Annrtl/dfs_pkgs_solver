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

fn solve_dependencies(
    modules: Vec<&Module>,
    top_module: String,
) -> Result<Vec<(String, Version)>, Vec<String>> {
    let mut graph = Graph::new();
    graph.loads_modules(modules);

    #[cfg(debug_assertions)]
    for (name, versions) in graph.vertex.iter() {
        for (version, vertice) in versions.iter() {
            println!("{}-{}: {:?}", name, version, vertice);
        }
    }

    let (top_name, top_version) = top_module.split_once(":").unwrap();
    let top_version = Version::parse(top_version).unwrap();

    match graph.dfs(top_name.to_string(), top_version) {
        Ok(result) => Ok(result),
        Err(messages) => Err(messages),
    }
}

fn main() {
    let mut modules: Vec<&Module> = Vec::new();

    let module = Module {
        name: "PMU".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![
            Requirement {
                module: "UART".to_string(),
                constraint: VersionReq::parse("^0.1.0").unwrap(),
            },
            Requirement {
                module: "I2C".to_string(),
                constraint: VersionReq::parse("^0.1.0").unwrap(),
            },
            Requirement {
                module: "DFF".to_string(),
                constraint: VersionReq::parse("^0.1.0").unwrap(),
            },
        ],
    };

    modules.push(&module);

    let module = Module {
        name: "PMU".to_string(),
        version: Version::parse("0.2.0").unwrap(),
        requirements: vec![
            Requirement {
                module: "UART".to_string(),
                constraint: VersionReq::parse("^0.5.0").unwrap(),
            },
            Requirement {
                module: "I2C".to_string(),
                constraint: VersionReq::parse("^0.2.0").unwrap(),
            },
            Requirement {
                module: "DFF".to_string(),
                constraint: VersionReq::parse("~0.2.0").unwrap(),
            },
        ],
    };

    modules.push(&module);

    let module = Module {
        name: "PMU".to_string(),
        version: Version::parse("0.3.0").unwrap(),
        requirements: vec![
            Requirement {
                module: "UART".to_string(),
                constraint: VersionReq::parse("^0.5.0").unwrap(), // ^0.2.0 not satisfied
            },
            Requirement {
                module: "I2C".to_string(),
                constraint: VersionReq::parse(">=0.2.0").unwrap(),
            },
            Requirement {
                module: "DFF".to_string(),
                constraint: VersionReq::parse("^0.2.1").unwrap(), // ^0.1.1 not satisfied
            },
        ],
    };

    modules.push(&module);

    let module = Module {
        name: "UART".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![Requirement {
            module: "DFF".to_string(),
            constraint: VersionReq::parse("~0.1.0").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "UART".to_string(),
        version: Version::parse("0.5.0").unwrap(),
        requirements: vec![
            Requirement {
                module: "DFF".to_string(),
                constraint: VersionReq::parse("^0.2.0").unwrap(),
            },
            //Requirement {
            //    module: "PMU".to_string(),
            //    constraint: VersionReq::parse("^0.3.0").unwrap(),
            //}
        ],
    };

    modules.push(&module);

    let module = Module {
        name: "I2C".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![Requirement {
            module: "DFF".to_string(),
            constraint: VersionReq::parse("^0.1.0").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "I2C".to_string(),
        version: Version::parse("0.2.0").unwrap(),
        requirements: vec![Requirement {
            module: "DFF".to_string(),
            constraint: VersionReq::parse("^0.2.0").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "I2C".to_string(),
        version: Version::parse("1.0.0").unwrap(),
        requirements: vec![Requirement {
            module: "DFF".to_string(),
            constraint: VersionReq::parse("^0.2.1").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "I2C".to_string(),
        version: Version::parse("1.0.1").unwrap(),
        requirements: vec![Requirement {
            module: "DFF".to_string(),
            constraint: VersionReq::parse("^0.2.1").unwrap(),
        }],
    };

    modules.push(&module);

    let module = Module {
        name: "DFF".to_string(),
        version: Version::parse("0.1.0").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    let module = Module {
        name: "DFF".to_string(),
        version: Version::parse("0.2.0").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    let module = Module {
        name: "DFF".to_string(),
        version: Version::parse("0.2.1").unwrap(),
        requirements: vec![],
    };

    modules.push(&module);

    // let top_module: String = "PMU:0.5.0".to_string();
    let top_module: String = "PMU:0.3.0".to_string();

    let result = match solve_dependencies(modules, top_module) {
        Ok(result) => result,
        Err(messages) => {
            println!("Error(s):\n  - {}", messages.join("\n  - "));
            return;
        }
    };

    for (module, version) in result.iter() {
        println!("Using: {}: {}", module, version);
    }
}
