use production_clicker::production::resource::{ResourceManager, setup_resource_manager};
use production_clicker::processing::recipe_loader::RecipeLoader;
use std::path::{Path, PathBuf};

fn recipe_path() -> PathBuf {
    let mut start = PathBuf::from("configurations");
    start.push("recipes.json");
    start
}

fn main() {
    let mut resource_manager = ResourceManager::new();
    setup_resource_manager(&mut resource_manager).expect("Couldn't set up resource manager");

    let mut recipe_loader = RecipeLoader::new(recipe_path(), &mut resource_manager);
    recipe_loader.load_recipes().expect("Couldn't load recipes from file");
}
