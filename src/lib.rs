pub mod gameobject;
pub use gameobject::{GameObject, TextGameObject, GameObjectTrait};

pub mod editor;
pub use editor::{Editor};

#[cfg(test)]
mod tests {
    use crate::{
        gameobject::{TextGameObject},
        editor::{Editor}
    };

    #[test]
    fn editor_test() {
        let mut editor = Editor::load_ws();
        let mut object = TextGameObject::from(Some("Hello, World!".to_string()), 5);
        editor.add_objects(&[object.clone()]);
        object.base.pos_x = 45.0;
        editor.add_objects(&[object.clone()]);

        println!("{:#?}", editor.save());
    }
}