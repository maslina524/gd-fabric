pub mod gameobject;
pub use gameobject::*;

pub mod editor;
pub use editor::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_objects() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        let obj1 = GameObject::new();
        
        let mut obj2 = TextGameObject::from(String::from("Hello, World"), 5);
        obj2.pos = Point::new(105., 105.);

        editor.add_objects(vec![obj1, obj2]);

        let save_result = editor.save();
        assert!(save_result.is_ok(), "Failed to save: {:?}", save_result.err());
    }

    #[test]
    fn get_level_string() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");
        
        match editor.level_string() {
            Ok(c) => assert_eq!(c.starts_with("kS38,"), true),
            Err(e) => panic!("unexpected err {e}"),
        }
        // 
    }

    #[test]
    fn editor_action_only() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        if let Err(ret) = editor.save() {
            panic!("unexpected err {ret}");
        }
    }

    #[test]
    fn groups() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        let mut obj = TextGameObject::from(String::from("I have 1st group"), 5);
        obj.groups.insert(1);

        let mut obj2 = TextGameObject::from(String::from("I have 2nd group"), 5);
        obj2.pos.y = -15.;
        obj2.groups.insert(2);

        let mut obj3 = TextGameObject::from(String::from("I have a 1st group and a 2nd parent group"), 5);
        obj3.pos.y = -45.;
        obj3.groups.insert(1);
        obj3.p_groups.insert(2);

        editor.add_objects(vec![obj, obj2, obj3]);
        if let Err(ret) = editor.save() {
            panic!("unexpected err {ret}");
        }
    }

    #[test]
    fn debug_groups() {
        let mut editor = Editor::load_ws()
            .expect("Failed to connect to editor. Make sure the mod is installed and running");

        editor.add_debug_group = true;  // enabled by default
        editor.clear_debug_objs = true; // enabled by default
        
        editor.add_objects(vec![GameObject::from(211, Point::new(45., 45.))]);

        if let Err(ret) = editor.save() {
            panic!("unexpected err {ret}");
        }
    }
}