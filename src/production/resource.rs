use iced::Color;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use regex::{Regex, Matches, Match, Captures};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum ResourceTag {
    Base,
    Metal,
    Ore,
    Ingot
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: u64,
    name: String,
    description: String,
    base_icon: PathBuf,
    fg_color: Color,
    bg_color: Color,
    tags: Vec<ResourceTag>,
}

impl Resource {
    pub fn new<
        'a,
        S1: AsRef<str>,
        S2: AsRef<str>,
        P: AsRef<Path>,
        I: IntoIterator<Item = &'a ResourceTag>,
    >(
        name: S1,
        description: S2,
        base_icon: P,
        fg_color: Color,
        bg_color: Color,
        tags: I,
    ) -> Self {
        Self {
            id: 0,
            name: name.as_ref().to_string(),
            description: description.as_ref().to_string(),
            base_icon: PathBuf::from(base_icon.as_ref()),
            fg_color,
            bg_color,
            tags: tags.into_iter().cloned().collect(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn base_icon(&self) -> &Path {
        self.base_icon.as_path()
    }

    pub fn fg_color(&self) -> &Color {
        &self.fg_color
    }

    pub fn bg_color(&self) -> &Color {
        &self.bg_color
    }

    pub fn tags(&self) -> &Vec<ResourceTag> {
        &self.tags
    }

    pub fn contains_tag(&self, tag: &ResourceTag) -> bool {
        self.tags.contains(tag)
    }

    pub fn contains_all_tags<'a, I: IntoIterator<Item = &'a ResourceTag>>(&self, tags: I) -> bool {
        for tag in tags {
            if !self.tags.contains(tag) {
                return false;
            }
        }
        true
    }
}

pub struct ResourceManager {
    resources: HashMap<u64, Resource>,
    processed_transformations: Vec<Box<dyn Fn(&Resource) -> Option<Resource>>>,
    resources_created: u64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: Default::default(),
            processed_transformations: vec![],
            resources_created: 0,
        }
    }

    pub fn add_processed_transformer<F>(&mut self, transformer: F) -> Result<(), String>
    where
        F: 'static + Fn(&Resource) -> Option<Resource>,
    {
        let mut to_add = Vec::new();
        for resource in self.resources.values() {
            if let Some(processed) = transformer(resource) {
                to_add.push(processed);
            }
        }
        for resource in to_add {
            self.add_resource(resource)?;
        }
        self.processed_transformations.push(Box::new(transformer));
        Ok(())
    }

    pub fn add_resource(&mut self, mut resource: Resource) -> Result<u64, String> {
        if self.resource_by_name(resource.name()).is_some() {
            return Err("Resource already exists".to_string());
        }
        let resource_id = self.resources_created;
        self.resources_created += 1;
        resource.id = resource_id;
        self.resources.insert(resource_id, resource);
        let resource = &self.resources[&resource_id];
        let mut to_add = Vec::new();
        for transformer in &self.processed_transformations {
            if let Some(mut processed) = transformer(resource) {
                to_add.push(processed);
            }
        }
        for resource in to_add {
            self.add_resource(resource)?;
        }
        Ok(resource_id)
    }

    pub fn resources(&self) -> impl Iterator<Item = &Resource> {
        self.resources.values()
    }

    pub fn resource_by_name<S: AsRef<str>>(&self, name: S) -> Option<&Resource> {
        let name = name.as_ref();
        self.resources.values().find(|res| res.name() == name)
    }

    pub fn resource_by_id(&self, id: u64) -> Option<&Resource> {
        self.resources.values().find(|res| res.id == id)
    }

    pub fn resource_id_by_name<S: AsRef<str>>(&self, name: S) -> Option<u64> {
        let name = name.as_ref();
        self.resources
            .values()
            .find(|res| res.name() == name)
            .map(|res| res.id)
    }

    pub fn resources_by_regular_expression(&self, regex: &Regex) -> Vec<(&Resource, Captures)> {
            self.resources()
                .filter_map(|resource| {
                    let name = resource.name();
                    if let Some(matched) = regex.captures(name.as_str()) {
                        Some((resource, matched))
                    } else {
                        None
                    }
                })
                .collect()
    }

    pub fn resources_with_tag(&self, tag: &ResourceTag) -> Vec<&Resource> {
        self.resources()
            .filter(|res| res.contains_tag(tag))
            .collect()
    }

    pub fn resources_with_tags<'a, I : IntoIterator<Item=&'a ResourceTag> + Clone>(&self, tags: I) -> Vec<&Resource> {
        self.resources()
            .filter(|res| res.contains_all_tags(tags.clone()))
            .collect()
    }
}

impl Debug for ResourceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceManager {{")?;
        write!(
            f,
            "{}",
            self.resources()
                .map(|res| res.name().clone())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        write!(f, "}}")
    }
}

pub fn setup_resource_manager(manager: &mut ResourceManager) -> Result<(), String> {
    manager.add_resource(Resource::new(
        "Iron",
        "Fe",
        "",
        Color::WHITE,
        Color::BLACK,
        &[ResourceTag::Metal, ResourceTag::Base],
    ))?;
    manager.add_resource(Resource::new(
        "Copper",
        "Fe",
        "",
        Color::WHITE,
        Color::BLACK,
        &[ResourceTag::Metal, ResourceTag::Base],
    ))?;
    manager.add_resource(Resource::new(
        "Gold",
        "Fe",
        "",
        Color::WHITE,
        Color::BLACK,
        &[ResourceTag::Metal, ResourceTag::Base],
    ))?;

    // transformers first
    manager.add_processed_transformer(|resource| {
        if resource.contains_all_tags(&[ResourceTag::Base, ResourceTag::Metal]) {
            Some(Resource::new(
                format!("{} Ingot", resource.name()),
                format!("A pure form of {} in a convenient bar form", resource.name),
                "",
                resource.fg_color,
                resource.bg_color,
                &[ResourceTag::Metal, ResourceTag::Ingot],
            ))
        } else {
            None
        }
    })?;
    manager.add_processed_transformer(|resource| {
        if resource.contains_all_tags(&[ResourceTag::Base, ResourceTag::Metal]) {
            Some(Resource::new(
                format!("{} Plate", resource.name()),
                format!("{} flattened to the MAXIMUM extent", resource.name),
                "",
                resource.fg_color,
                resource.bg_color,
                &[ResourceTag::Metal],
            ))
        } else {
            None
        }
    })?;
    manager.add_processed_transformer(|resource| {
        if resource.contains_all_tags(&[ResourceTag::Base, ResourceTag::Metal]) {
            Some(Resource::new(
                format!("{} Ore", resource.name()),
                format!("The ore form of {}", resource.name),
                "",
                resource.fg_color,
                resource.bg_color,
                &[ResourceTag::Metal, ResourceTag::Ore],
            ))
        } else {
            None
        }
    })?;
    manager.add_processed_transformer(|resource| {
        if resource.contains_all_tags(&[ResourceTag::Base, ResourceTag::Metal]) {
            Some(Resource::new(
                format!("{} Wire", resource.name()),
                format!("{} thin and noodly, just the way I like it", resource.name),
                "",
                resource.fg_color,
                resource.bg_color,
                &[ResourceTag::Metal],
            ))
        } else {
            None
        }
    })?;
    /*
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

     */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformations_work() {
        let mut manager = ResourceManager::new();
        setup_resource_manager(&mut manager).unwrap();
        println!("{:?}", manager);
        assert!(
            manager.resource_by_name("Iron").is_some(),
            "Iron wasn't created"
        );
        assert!(manager.resource_by_name("Iron Plate").is_some());
        assert!(manager.resource_by_name("Iron Ingot").is_some());
    }
}
