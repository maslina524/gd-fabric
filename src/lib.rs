pub mod gameobject;
pub use gameobject::{GameObject, TextGameObject};

pub mod editor;
pub use editor::{Editor};

#[cfg(test)]
mod tests {
    use crate::{
        GameObject, Editor, TextGameObject
    };

    #[test]
    fn add_objects() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        let obj1 = GameObject::new();
        
        let mut obj2 = TextGameObject::from(String::from("Hello, World"), 5);
        obj2.pos_x = 105.0;
        obj2.pos_y = 105.0;

        editor.add_objects(vec![obj1, obj2]);

        let save_result = editor.save();
        assert!(save_result.is_ok(), "Failed to save: {:?}", save_result.err());
    }

    #[test]
    fn get_level_string() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");
        println!("{:#?}", editor.level_string());
        // assert_eq!(editor.level_string().unwrap().starts_with("kS38,"), true)
    }

    #[test]
    fn editor_action_only() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        println!("{:#?}", editor.save())
    }
}