use semver::Version;
use std::collections::{BTreeMap, HashMap};

use crate::{Module, Requirement};

pub struct Graph {
    pub vertex: HashMap<String, HashMap<Version, Vertice>>,
}

#[derive(Debug, Clone)]
pub struct Vertice {
    parents: BTreeMap<String, Vec<Version>>,
    children: BTreeMap<String, Vec<Version>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertex: HashMap::new(),
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
        let vertice = Vertice::new();
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
            self.add_edge_from_requirement(
                module.name.clone(),
                module.version.clone(),
                requirement,
            );
        }
    }

    fn add_edge_from_requirement(
        &mut self,
        name: String,
        version: Version,
        requirement: &Requirement,
    ) {
        // Prepare the list of children to add
        let mut children_to_add: Vec<(String, Version)> = Vec::new();
        for (vertice_name, vertice_versions) in self.vertex.iter() {
            for vertice_version in vertice_versions.keys() {
                if (vertice_name.to_string() == requirement.module)
                    && requirement.constraint.matches(vertice_version)
                {
                    children_to_add.push((vertice_name.clone(), vertice_version.clone()));
                }
            }
        }

        // Add the prepared children to the graph
        for (vertice_name, vertice_versions) in self.vertex.iter_mut() {
            for (vertice_version, child_vertice) in vertice_versions.iter_mut() {
                // Add parent to the child
                if children_to_add.contains(&(vertice_name.clone(), vertice_version.clone())) {
                    if child_vertice.parents.contains_key(&name) {
                        let parents = child_vertice.parents.get_mut(&name).unwrap();
                        parents.push(version.clone());
                    } else {
                        child_vertice
                            .parents
                            .insert(name.clone(), vec![version.clone()]);
                    }
                }

                // Add child to the parent
                if vertice_name == &name && vertice_version == &version {
                    for (subs_child_name, subs_child_version) in &children_to_add {
                        if child_vertice.children.contains_key(subs_child_name) {
                            let versions = child_vertice.children.get_mut(subs_child_name).unwrap();
                            versions.push(subs_child_version.clone());
                        } else {
                            child_vertice
                                .children
                                .insert(subs_child_name.clone(), vec![subs_child_version.clone()]);
                        }
                    }
                }
            }
        }
    }

    fn sort_children(&mut self) {
        for (_, versions) in self.vertex.iter_mut() {
            for (_, vertice) in versions.iter_mut() {
                vertice.sort_children();
            }
        }
    }

    fn dfs_recursive_versions(
        &self,
        visited: &mut BTreeMap<String, Version>,
        name: String,
        versions: &Vec<Version>,
    ) -> Result<String, String> {
        // For each version of the dependency module
        for version in versions {
            // If a module version is already visited
            if visited.contains_key(&name) {
                if !visited.get(&name).unwrap().eq(&version) {
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

    fn dfs_recursive(
        &self,
        visited: &mut BTreeMap<String, Version>,
        vertice: Vertice,
    ) -> Result<String, String> {
        // No child
        if vertice.children.is_empty() {
            return Ok("".to_string());
        }

        // For each dependencuy module
        for (name, versions) in &vertice.children {
            match self.dfs_recursive_versions(visited, name.clone(), versions) {
                Ok(_) => continue,
                Err(_) => return Err("".to_string()),
            }
        }
        return Ok("".to_string());
    }

    pub fn dfs(
        &mut self,
        top_module: String,
        top_version: Version,
    ) -> Result<Vec<(String, Version)>, String> {
        self.sort_children();
        let mut visited: BTreeMap<String, Version> = BTreeMap::new();
        visited.insert(top_module.clone(), top_version.clone());
        let top_vertice = self
            .vertex
            .get(&top_module)
            .unwrap()
            .get(&top_version)
            .unwrap();
        match self.dfs_recursive(&mut visited, top_vertice.clone()) {
            Ok(_) => {
                let mut result: Vec<(String, Version)> = Vec::new();
                for (name, version) in visited.iter() {
                    result.push((name.clone(), version.clone()));
                }
                Ok(result)
            }
            Err(_) => Err("".to_string()),
        }
    }
}

impl Vertice {
    fn new() -> Vertice {
        Vertice {
            parents: BTreeMap::new(),
            children: BTreeMap::new(),
        }
    }

    pub fn sort_children(&mut self) {
        for (_, child) in self.children.iter_mut() {
            child.sort_by(|a, b| b.cmp(a));
        }
    }
}
