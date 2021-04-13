use crate::processing::recipe::{Recipe, RecipePattern};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::production::resource::ResourceManager;
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug)]
pub struct RecipeLoader<'a> {
    file_path: PathBuf,
    created_recipes: HashMap<String, Recipe>,
    resource_manager: &'a ResourceManager
}

impl<'a> RecipeLoader<'a> {
    pub fn new<P: AsRef<Path>>(file_path: P, manager: &'a ResourceManager) -> Self {
        RecipeLoader {
            file_path: PathBuf::from(file_path.as_ref()),
            created_recipes: Default::default(),
            resource_manager: manager
        }
    }

    pub fn load_recipes(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let recipes: HashMap<String, RecipePattern> = serde_json::from_reader(reader)?;
        //self.created_recipes.extend(recipes);
        println!("{:?}", recipes);
        let new_recipes = recipes
            .into_iter()
            .flat_map(|(key, val)|
                val.into_recipes(self.resource_manager)
                    .into_iter()
                    .map(|recipe| (key.clone(), recipe))
                    .collect::<Vec<(_, _)>>()
            )
            .collect::<Vec<_>>();
        self.created_recipes.extend(new_recipes);
        Ok(())
    }
}


