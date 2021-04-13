use crate::production::resource::{ResourceManager, ResourceTag};
use regex::Regex;
use std::collections::HashMap;
use serde_json::Value;
use std::ops::Deref;

#[derive(Debug, PartialEq, Deserialize)]
pub struct RecipeComponent {
    resource_id: u64,
    quantity: usize,
}

impl RecipeComponent {
    pub const fn new(resource_id: u64, quantity: usize) -> Self {
        RecipeComponent {
            resource_id,
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct Recipe {
    inputs: Vec<RecipeComponent>,
    outputs: Vec<RecipeComponent>,
    base_time: u16,
    requirements: Vec<()>
}

impl Recipe {
    pub fn new(inputs: Vec<RecipeComponent>, outputs: Vec<RecipeComponent>, base_time: u16, requirements: Vec<()>) -> Self {
        Recipe { inputs, outputs, base_time, requirements }
    }
}

#[derive(Debug, Deserialize)]
pub struct RecipePatternComponent {
    resource: Value,
    quantity: usize
}

impl RecipePatternComponent {

    pub fn pattern(&self) -> &Value {
        &self.resource
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }
}

#[derive(Debug, Deserialize)]
pub struct RecipePattern {
    input: Vec<RecipePatternComponent>,
    output: Vec<RecipePatternComponent>,
    base_time: u16,
    requirements: Vec<String>
}

impl RecipePattern {

    pub fn into_recipes(self, manager: &ResourceManager) -> Vec<Recipe> {
        println!("{:?}", self);
        let inputs_raw = self.input
            .iter()
            .map(|RecipePatternComponent{resource, quantity}| {
                match resource {
                    Value::String(name) => {
                        vec![manager.resource_by_name(name)
                            .expect(format!("No resource with name {}", name).as_str())]
                    }
                    Value::Object(dict) => {
                        let mut resources = manager.resources().collect::<Vec<_>>();
                        if dict.contains_key("tags") {
                            if let Value::Array(tags) = &dict["tags"] {
                                let tags = tags.iter()
                                    .map(|tag| {
                                        let tag_string = tag.as_str().expect("Tag must string");
                                        let tag: ResourceTag = serde_json::from_str(format!("\"{}\"", tag_string).as_str()).unwrap();
                                        tag
                                    })
                                    .collect::<Vec<_>>();
                                resources =
                                    resources.into_iter().filter(
                                        |res| res.contains_all_tags(tags.iter())
                                    )
                                        .collect();
                            } else {
                                panic!("Invalid entry for tags")
                            }
                        }
                        resources
                    }
                    _ => panic!("Invalid resource")
                }
            })
            .collect::<Vec<_>>();

        println!("{:?}", inputs);

        todo!()
    }
}

fn super_set_iterator<I, T>(input: I) -> Vec<Vec<T>> {
    
}



