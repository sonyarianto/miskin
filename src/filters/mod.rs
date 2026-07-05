pub mod cargo;
pub mod curl;
pub mod docker;
pub mod files;
pub mod generic;
pub mod gh;
pub mod git;
pub mod lint;
pub mod npm;
pub mod system;
pub mod tests;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FilterResult {
    PassThrough(String),
    Filtered(String),
    Silent,
}

pub trait CommandFilter: Send + Sync {
    fn name(&self) -> &str;
    fn aliases(&self) -> &[&str] {
        &[]
    }
    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult;
}

pub struct FilterRegistry {
    filters: Vec<Box<dyn CommandFilter>>,
    alias_map: HashMap<String, usize>,
}

impl FilterRegistry {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            alias_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, filter: Box<dyn CommandFilter>) {
        let idx = self.filters.len();
        self.alias_map
            .insert(filter.name().to_string(), idx);
        for alias in filter.aliases() {
            self.alias_map.insert(alias.to_string(), idx);
        }
        self.filters.push(filter);
    }

    pub fn get(&self, command: &str) -> Option<&dyn CommandFilter> {
        self.alias_map
            .get(command)
            .map(|&idx| self.filters[idx].as_ref())
    }

    #[allow(dead_code)]
    pub fn has(&self, command: &str) -> bool {
        self.alias_map.contains_key(command)
    }

    #[allow(dead_code)]
    pub fn names(&self) -> Vec<&str> {
        self.filters.iter().map(|f| f.name()).collect()
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(git::GitFilter));
        registry.register(Box::new(gh::GhFilter));
        registry.register(Box::new(cargo::CargoFilter));
        registry.register(Box::new(docker::DockerFilter));
        registry.register(Box::new(files::FilesFilter));
        registry.register(Box::new(npm::NpmFilter));
        registry.register(Box::new(tests::TestsFilter));
        registry.register(Box::new(lint::LintFilter));
        registry.register(Box::new(curl::CurlFilter));
        registry.register(Box::new(system::SystemFilter));
        registry
    }
}
