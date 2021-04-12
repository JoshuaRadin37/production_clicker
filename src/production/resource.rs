use iced::Color;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct BaseResource {
    id: u64,
    name: String,
    description: String,
    base_icon: PathBuf,
    fg_color: Color,
    bg_color: Color,
}

impl BaseResource {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, P: AsRef<Path>>(
        name: S1,
        description: S2,
        base_icon: P,
        fg_color: Color,
        bg_color: Color,
    ) -> Self {
        BaseResource {
            id: 0,
            name: name.as_ref().to_string(),
            description: description.as_ref().to_string(),
            base_icon: PathBuf::from(base_icon.as_ref()),
            fg_color,
            bg_color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessedResource {
    id: u64,
    prefix: Option<String>,
    suffix: Option<String>,
    base: BaseResource,
    description: String,
    base_icon: PathBuf,
}

impl ProcessedResource {
    pub fn new<S3: AsRef<str>, P: AsRef<Path>>(
        prefix: Option<&str>,
        suffix: Option<&str>,
        base: BaseResource,
        description: S3,
        base_icon: P,
    ) -> Self {
        ProcessedResource {
            id: 0,
            prefix: prefix.map(|s| s.to_string()),
            suffix: suffix.map(|s| s.to_string()),
            base,
            description: description.as_ref().to_string(),
            base_icon: PathBuf::from(base_icon.as_ref()),
        }
    }
}

pub trait Resource {
    fn id(&self) -> u64;
    fn name(&self) -> String;
    fn description(&self) -> &String;
    fn base_icon(&self) -> &Path;
    fn fg_color(&self) -> &Color;
    fn bg_color(&self) -> &Color;
}

impl Resource for BaseResource {
    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn base_icon(&self) -> &Path {
        self.base_icon.as_path()
    }

    fn fg_color(&self) -> &Color {
        &self.fg_color
    }

    fn bg_color(&self) -> &Color {
        &self.bg_color
    }
}

impl Resource for ProcessedResource {
    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> String {
        format!(
            "{}{}{}",
            self.prefix.as_ref().map_or("", |o| o.as_str()),
            self.base.name,
            self.suffix.as_ref().map_or("", |o| o.as_str())
        )
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn base_icon(&self) -> &Path {
        self.base_icon.as_path()
    }

    fn fg_color(&self) -> &Color {
        self.base.fg_color()
    }

    fn bg_color(&self) -> &Color {
        self.base.bg_color()
    }
}

pub struct ResourceManager {
    base_resources: HashMap<u64, BaseResource>,
    processed_resources: HashMap<u64, ProcessedResource>,
    processed_transformations: Vec<Box<dyn Fn(&BaseResource) -> ProcessedResource>>,
    resources_created: u64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            base_resources: Default::default(),
            processed_resources: Default::default(),
            processed_transformations: vec![],
            resources_created: 0,
        }
    }

    pub fn add_processed_transformer<F>(&mut self, transformer: F)
    where
        F: 'static + Fn(&BaseResource) -> ProcessedResource,
    {
        for resource in self.base_resources.values() {
            let mut processed: ProcessedResource = transformer(resource);
            let resource_id = self.resources_created;
            self.resources_created += 1;
            processed.id = resource_id;
            self.processed_resources.insert(resource_id, processed);
        }
        self.processed_transformations.push(Box::new(transformer));
    }

    pub fn add_resource(&mut self, mut resource: BaseResource) -> Result<u64, String> {
        if self.resource_by_name(resource.name()).is_some() {
            return Err("Resource already exists".to_string());
        }
        let resource_id = self.resources_created;
        self.resources_created += 1;
        resource.id = resource_id;
        self.base_resources.insert(resource_id, resource);
        let resource = &self.base_resources[&resource_id];
        for transformer in &self.processed_transformations {
            let mut processed: ProcessedResource = transformer(resource);
            let resource_id = self.resources_created;
            self.resources_created += 1;
            processed.id = resource_id;
            self.processed_resources.insert(resource_id, processed);
        }
        Ok(resource_id)
    }

    pub fn resource_by_name<S: AsRef<str>>(&self, name: S) -> Option<&dyn Resource> {
        let name = name.as_ref();
        if let Some(output) = self.base_resources.values().find(|res| res.name() == name) {
            Some(output)
        } else if let Some(output) = self
            .processed_resources
            .values()
            .find(|res| res.name() == name)
        {
            Some(output)
        } else {
            None
        }
    }

    pub fn resource_by_id(&self, id: u64) -> Option<&dyn Resource> {
        if let Some(output) = self.base_resources.values().find(|res| res.id == id) {
            Some(output)
        } else if let Some(output) = self.processed_resources.values().find(|res| res.id == id) {
            Some(output)
        } else {
            None
        }
    }

    pub fn resource_id_by_name<S: AsRef<str>>(&self, name: S) -> Option<u64> {
        let name = name.as_ref();
        if let Some(output) = self.base_resources.values().find(|res| res.name() == name) {
            Some(output.id)
        } else if let Some(output) = self
            .processed_resources
            .values()
            .find(|res| res.name() == name)
        {
            Some(output.id)
        } else {
            None
        }
    }
}

pub fn setup_resource_manager(manager: &mut ResourceManager) {
    // transformers first
    manager.add_processed_transformer(|resource| {
        ProcessedResource::new(None, Some("Ingot"), resource.clone(), "An ingot", "")
    });
    manager.add_processed_transformer(|resource| {
        ProcessedResource::new(None, Some("Ore"), resource.clone(), "The ore form!", "")
    });
    manager.add_processed_transformer(|resource| {
        ProcessedResource::new(
            None,
            Some("Plate"),
            resource.clone(),
            "Flattened to the MAXIMUM extent",
            "",
        )
    });

    manager.add_resource(BaseResource::new(
        "Iron",
        "Fe",
        "",
        Color::WHITE,
        Color::BLACK,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformations_work() {
        let mut manager = ResourceManager::new();
        setup_resource_manager(&mut manager);
        assert!(manager.resource_by_name("Iron").is_some(), "Iron wasn't created");
        assert!(manager.resource_by_name("Iron Plate").is_some());
        assert!(manager.resource_by_name("Iron Ingot").is_some());
    }
}
