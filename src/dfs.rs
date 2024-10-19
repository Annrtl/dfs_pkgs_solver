
use std::collections::HashMap;
use semver::Version;

use crate::{Module, Requirement};

pub struct Graph {
    pub vertex: HashMap<String, HashMap<Version, Vertice>>,
    //vertex: HashMap<(String, Version), Vertice>
}

#[derive(Debug)]
pub struct Vertice {
    name: String,
    version: Version,
    parents: Vec<(String, Version)>,
    children: Vec<(String, Version)>,
}

impl Graph {

    pub fn new() -> Graph {
        Graph {
            vertex: HashMap::new()
        }
    }

    pub fn loads_modules(&mut self, modules: Vec<&Module>) {
        for module in &modules {
            self.add_vertice_from_module(module);
        }
        for module in &modules {
            self.add_edges_from_module(module);
        }
    }

    fn add_vertice_from_module(&mut self, module: &Module) {
        let vertice = Vertice::new(module.name.clone(), module.version.clone());
        if self.vertex.contains_key(&module.name) {
            let versions_vertice = self.vertex.get_mut(&module.name).unwrap();
            versions_vertice.insert(module.version.clone(), vertice);
        } else {
            let mut versions_vertice = HashMap::new();
            versions_vertice.insert(module.version.clone(), vertice);
            self.vertex.insert(module.name.clone(), versions_vertice);
        }
    }

    fn add_edges_from_module(&mut self, module: &Module) {
        for requirement in &module.requirements {
            self.add_edge_from_requirement(module.name.clone(), module.version.clone(), requirement);
        }
    }

    fn add_edge_from_requirement(&mut self, name: String, version: Version, requirement: &Requirement) {
        let mut children: Vec<(String, Version)> = Vec::new();
        for (child_name, child_versions) in self.vertex.iter() {
            for child_version in child_versions.keys() {
                if requirement.constraint.matches(child_version) {
                    children.push((child_name.clone(), child_version.clone()));
                }
            }
        }
        for (child_name, child_versions) in self.vertex.iter_mut() {
            for (child_version, child_vertice) in child_versions.iter_mut() {
                if children.contains(&(child_name.clone(), child_version.clone())) {
                    child_vertice.parents.push((name.clone(), version.clone()));
                }
                if child_name == &name && child_version == &version {
                    for child in &children {
                        child_vertice.children.push(child.clone());
                    }
                }
            }
        }
    }

}

impl Vertice {
    fn new(name: String, version: Version) -> Vertice {
        Vertice {
            name,
            version,
            parents: Vec::new(),
            children: Vec::new()
        }
    }
}