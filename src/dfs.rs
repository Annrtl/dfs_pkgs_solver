use indexmap::IndexMap;
use semver::Version;
use std::collections::{BTreeMap, HashMap};

use crate::{Module, Requirement};

pub struct Graph {
    pub vertex: HashMap<String, HashMap<Version, Vertice>>,
}

#[derive(Debug, Clone)]
pub struct Vertice {
    name: String,
    version: Version,
    parents: BTreeMap<String, Vec<Version>>,
    children: BTreeMap<String, Vec<Version>>,
    requirements: Vec<Requirement>,
    unsatisfied_requirements: Vec<Requirement>,
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
        self.update_vertices();
    }

    fn add_vertice_from_module(&mut self, module: &Module) {
        let vertice = Vertice::new(
            module.name.clone(),
            module.version.clone(),
            module.requirements.clone(),
        );
        if self.vertex.contains_key(&module.name) {
            let versions_vertice = match self.vertex.get_mut(&module.name) {
                Some(versions) => versions,
                None => return,
            };
            versions_vertice.insert(module.version.clone(), vertice);
        } else {
            let mut versions_vertice = HashMap::new();
            versions_vertice.insert(module.version.clone(), vertice);
            self.vertex.insert(module.name.clone(), versions_vertice);
        }
    }

    fn update_vertices(&mut self) {
        let copy_of_graph_vertex = self.vertex.clone();
        for (_, versions) in self.vertex.iter_mut() {
            for (_, vertice) in versions.iter_mut() {
                vertice.add_children_from_graph(copy_of_graph_vertex.clone());
            }
        }

        let mut parents_to_add: Vec<(String, Version, String, Version)> = Vec::new();
        for (_, versions) in self.vertex.iter() {
            for (_, vertice) in versions.iter() {
                let mut vertice_parents_to_add = vertice.get_parents_to_add_list();
                parents_to_add.append(&mut vertice_parents_to_add);
            }
        }

        for (name, version, parent_name, parent_version) in parents_to_add {
            let vertice_versions = match self.vertex.get_mut(&name) {
                Some(versions) => versions,
                None => continue,
            };

            let vertice = match vertice_versions.get_mut(&version) {
                Some(vertice) => vertice,
                None => continue,
            };

            vertice.add_parents(parent_name, parent_version);
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
        visited: &mut IndexMap<String, Version>,
        visiting: &mut Vec<String>,
        name: String,
        versions: &Vec<Version>,
    ) -> Result<(), Vec<String>> {
        // Create errors vector
        let mut errors: Vec<String> = Vec::new();

        // Check for cycles
        if visiting.contains(&name) {
            let message: String = format!("Cycle detected: {} -> {}", visiting.join(" -> "), name);
            errors.push(message);
            return Err(errors);
        } else {
            visiting.push(name.clone());
        }
        // For each version of the dependency module
        for version in versions {
            // If a module version is already visited
            match visited.get(&name) {
                Some(visited_version) => {
                    if visited_version.eq(&version) {
                        continue;
                    }
                }
                None => {}
            }

            let child_vertice_versions = match self.vertex.get(&name) {
                Some(versions) => versions,
                None => {
                    errors.push(format!("Module {} not found", name));
                    visiting.pop();
                    return Err(errors);
                }
            };

            let child_vertice = match child_vertice_versions.get(&version) {
                Some(vertice) => vertice,
                None => {
                    errors.push(format!("Module {}:{} not found", name, version));
                    visiting.pop();
                    return Err(errors);
                }
            };

            match self.dfs_recursive(visited, visiting, child_vertice.clone()) {
                Ok(_) => {
                    visiting.pop();
                    // Select a version for this module
                    visited.insert(name.clone(), version.clone());

                    #[cfg(debug_assertions)]
                    println!("Added {:?}", name);
                    #[cfg(debug_assertions)]
                    println!("Visited {:?}", visited);

                    return Ok(());
                }
                Err(messages) => {
                    errors.push(messages.iter().map(|x| x.clone()).collect());
                    visiting.pop();
                    visited.shift_remove(&name);

                    #[cfg(debug_assertions)]
                    println!("Removed {:?}", name);
                    #[cfg(debug_assertions)]
                    println!("Visited {:?}", visited);
                }
            }
        }
        return Err(errors);
    }

    fn dfs_recursive(
        &self,
        visited: &mut IndexMap<String, Version>,
        visiting: &mut Vec<String>,
        vertice: Vertice,
    ) -> Result<(), Vec<String>> {
        // Unsatisfied vertice
        if !vertice.is_satisfied() {
            let unsatisfied_requirements = vertice.get_unsatisfied_requirements_string();
            let message: String = format!(
                "Vertice requirement of {}:{} not satisfied:\n{}",
                vertice.name, vertice.version, unsatisfied_requirements
            );
            return Err(vec![message]);
        }

        // Satisfied but no children
        if vertice.children.is_empty() {
            return Ok(());
        }

        // For each dependency module
        for (name, versions) in &vertice.children {
            match self.dfs_recursive_versions(visited, visiting, name.clone(), versions) {
                Ok(_) => continue,
                Err(messages) => return Err(messages),
            }
        }
        return Ok(());
    }

    pub fn dfs(
        &mut self,
        top_module: String,
        top_version: Version,
    ) -> Result<Vec<(String, Version)>, Vec<String>> {
        self.sort_children();
        let mut visited: IndexMap<String, Version> = IndexMap::new();
        let mut visiting: Vec<String> = Vec::new();
        visiting.push(top_module.clone());
        let top_vertice_versions = match self.vertex.get(&top_module) {
            Some(versions) => versions,
            None => return Err(vec![format!("Top module {} not found", top_module)]),
        };

        let top_vertice = match top_vertice_versions.get(&top_version) {
            Some(vertice) => vertice,
            None => {
                return Err(vec![format!(
                    "Top module {}:{} not found",
                    top_module, top_version
                )])
            }
        };

        match self.dfs_recursive(&mut visited, &mut visiting, top_vertice.clone()) {
            Ok(_) => {
                visited.insert(top_module.clone(), top_version.clone());
                let mut result: Vec<(String, Version)> = Vec::new();
                for (name, version) in visited.iter() {
                    result.push((name.clone(), version.clone()));
                }
                Ok(result)
            }
            Err(messages) => Err(messages),
        }
    }
}

impl Vertice {
    fn new(name: String, version: Version, requirements: Vec<Requirement>) -> Vertice {
        Vertice {
            name: name,
            version: version,
            parents: BTreeMap::new(),
            children: BTreeMap::new(),
            requirements: requirements,
            unsatisfied_requirements: Vec::new(),
        }
    }

    pub fn sort_children(&mut self) {
        for (_, child) in self.children.iter_mut() {
            child.sort_by(|a, b| b.cmp(a));
        }
    }

    fn add_children_from_graph(&mut self, vertex: HashMap<String, HashMap<Version, Vertice>>) {
        for requirement in self.requirements.clone() {
            let requirement_vertex = match vertex.get(&requirement.module) {
                Some(versions) => versions,
                None => {
                    self.unsatisfied_requirements.push(requirement);
                    continue;
                }
            };
            match self.add_children_from_requirement(requirement.clone(), requirement_vertex) {
                Ok(_) => continue,
                Err(_) => {
                    self.unsatisfied_requirements.push(requirement);
                }
            }
        }
    }

    fn add_children_from_requirement(
        &mut self,
        requirement: Requirement,
        vertex: &HashMap<Version, Vertice>,
    ) -> Result<(), ()> {
        let mut satisfied = false;

        for (version, vertice) in vertex.iter() {
            if requirement.constraint.matches(version) {
                self.add_children(vertice.name.clone(), version.clone());
                satisfied = true;
            }
        }

        if satisfied {
            Ok(())
        } else {
            Err(())
        }
    }

    fn add_children(&mut self, name: String, version: Version) {
        match self.children.get_mut(&name) {
            Some(versions) => {
                versions.push(version);
            }
            None => {
                self.children.insert(name, vec![version]);
            }
        }
    }

    fn get_parents_to_add_list(&self) -> Vec<(String, Version, String, Version)> {
        let mut parents_to_add: Vec<(String, Version, String, Version)> = Vec::new();
        for (name, versions) in &self.children {
            for version in versions {
                parents_to_add.push((
                    name.clone(),
                    version.clone(),
                    self.name.clone(),
                    self.version.clone(),
                ));
            }
        }
        parents_to_add
    }

    fn add_parents(&mut self, name: String, version: Version) {
        match self.parents.get_mut(&name) {
            Some(versions) => {
                versions.push(version);
            }
            None => {
                self.parents.insert(name, vec![version]);
            }
        }
    }

    fn is_satisfied(&self) -> bool {
        self.unsatisfied_requirements.is_empty()
    }

    fn get_unsatisfied_requirements_string(&self) -> String {
        let mut result = String::new();
        for requirement in &self.unsatisfied_requirements {
            result.push_str(&format!(
                "\t{}: {}\n",
                requirement.module, requirement.constraint
            ));
        }
        result
    }
}
