
use std::collections::HashMap;
use semver::Version;

use crate::{Module, Requirement};

pub struct Graph {
    pub vertex: HashMap<String, HashMap<Version, Vertice>>,
    //vertex: HashMap<(String, Version), Vertice>
}

#[derive(Debug, Clone)]
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

    // TODO: Remove pub
    pub fn sort_children(&mut self) {
        for (_, versions) in self.vertex.iter_mut() {
            for (_, vertice) in versions.iter_mut() {
                vertice.sort_children();
            }
        }
    }

    fn dfs_recursive(&self, visited: &mut HashMap<String, Version>, vertice: Vertice) -> Result<String, String>{
        // No child
        if vertice.children.is_empty() {
            return Ok("".to_string());
        }

        for (name, version) in vertice.children {
            if visited.contains_key(&name) {
                if ! visited.get(&name).unwrap().eq(&version) {
                    continue;
                }
                visited.get_mut(&name).unwrap().clone_from(&version);
            } else {
                visited.insert(name.clone(), version.clone());
            }

            let child_vertice = self.vertex.get(&name).unwrap().get(&version).unwrap();
            match self.dfs_recursive(visited, child_vertice.clone()) {
                Ok(_) => return Ok("".to_string()),
                Err(_) => {
                    visited.remove(&name);
                }
            }
        }
        return Err("".to_string());
    }

    pub fn dfs(&self, top_module: String, top_version: Version) -> Result<Vec<(String, Version)>, String> {
        let mut visited: HashMap<String, Version> = HashMap::new();
        visited.insert(top_module.clone(), top_version.clone());
        let top_vertice = self.vertex.get(&top_module).unwrap().get(&top_version).unwrap();
        match self.dfs_recursive(&mut visited, top_vertice.clone()) {
            Ok(_) => {
                let mut result: Vec<(String, Version)> = Vec::new();
                for (name, version) in visited.iter() {
                    result.push((name.clone(), version.clone()));
                }
                Ok(result)
            },
            Err(_) => Err("".to_string())
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
    
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| b.cmp(a));
    }

}