use core::fmt;
use base64::{Engine as _, engine::general_purpose};

#[derive(Clone)]
pub enum SingleColorType {
    Default = 0,
    Base = 1,
    Detail = 2
}

#[derive(Clone)]
pub struct HsvValue {
    h: f32,
    s: f32,
    v: f32,
    s_checked: bool,
    v_checked: bool,
}
impl HsvValue {
    pub fn new() -> Self {
        Self {
            h: 0.,
            s: 1.,
            v: 1.,
            s_checked: false,
            v_checked: false
        }
    }
 
    pub fn serialize(&self) -> String {
        format!("{}a{}a{}a{}a{}", self.h, self.s, self.v, self.s_checked as u8, self.v_checked as u8)
    }
}

macro_rules! prop {
    (bool $value:expr, $key:expr, $def:expr) => {
        {
            let val = $value;
            let def = $def;
            if val != def {
                format!("{},{}," , $key, val as u8)
            } else {
                String::new()
            }
        }
    };
    
    (f32 $value:expr, $key:expr, $def:expr) => {
        {
            let val = $value;
            let def = $def;
            if (val - def).abs() > f32::EPSILON {
                format!("{},{}," , $key, val)
            } else {
                String::new()
            }
        }
    };
    
    ($value:expr, $key:expr, $def:expr) => {
        {
            let val = &$value;
            let def = &$def;
            if val != def {
                format!("{},{}," , $key, val)
            } else {
                String::new()
            }
        }
    };
}

pub trait GameObjectTrait {
    fn serialize(&self) -> String;
}

#[derive(Clone)]
pub struct GameObject {
    pub id: u16,
    pub pos_x: f32,
    pub pos_y: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub scale: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub warp_x: f32,
    pub warp_y: f32,
    pub color_1: u16,
    pub color_2: u16,
    pub color_typ: SingleColorType,
    pub hsv_1: HsvValue,
    pub hsv_2: HsvValue,
}

impl GameObject {
    pub fn new() -> Self {
        Self {
            id: 1,
            pos_x: 15.,
            pos_y: 15.,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            scale: 1.,
            scale_x: 1.,
            scale_y: 1.,
            warp_x: 0.,
            warp_y: 0.,
            color_1: 0, // 0 is default color
            color_2: 0,
            color_typ: SingleColorType::Default,
            hsv_1: HsvValue::new(),
            hsv_2: HsvValue::new(),
        }
    }

    fn base_serialize(&self) -> String {
        let mut ret = String::new();

        ret.push_str(format!("1,{},2,{},3,{},", self.id, self.pos_x, self.pos_y).as_str());
        ret.push_str(&prop!(self.rotation, 6, 0.));
        ret.push_str(&prop!(bool self.flip_x, 4, false));
        ret.push_str(&prop!(bool self.flip_y, 5, false));
        ret.push_str(&prop!(self.scale, 32, 1.));
        ret.push_str(&prop!(self.scale_x, 128, 1.));
        ret.push_str(&prop!(self.scale_y, 129, 1.));
        ret.push_str(&prop!(self.warp_x, 131, 0.));
        ret.push_str(&prop!(self.warp_y, 132, 0.));
        ret.push_str(&prop!(self.color_1, 21, 0));
        ret.push_str(&prop!(self.color_2, 22, 0));
        ret.push_str(&prop!(self.color_typ.clone() as u8, 497, 0));

        let hsv_def = HsvValue::new().serialize();
        if self.hsv_1.serialize() != hsv_def { ret.push_str("41,1,"); }
        ret.push_str(&prop!(self.hsv_1.serialize(), 43, hsv_def));
        if self.hsv_2.serialize() != hsv_def { ret.push_str("42,1,"); }
        ret.push_str(&prop!(self.hsv_2.serialize(), 44, hsv_def));

        ret
    }

    /// Serializes a GameObject (save string)
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use crate::GameObject;
    /// let obj = GameObject::new();
    /// let serialized = obj.serialize();
    /// assert_eq!(serialized, "1,1,2,15,3,15;");
    /// ```
    /// 
    /// # Returns
    /// 
    /// A string in the format "key,value,key,value,...;"
    pub fn serialize(&self) -> String {
        let mut ret: String = self.base_serialize();

        ret.pop();
        ret.push(';');
        ret
    }
}

#[derive(Clone)]
pub struct TextGameObject {
    pub base: GameObject,
    pub text: String,
    pub kerning: i32,
}

impl TextGameObject {
    pub fn new() -> Self {
        let mut base = GameObject::new();
        base.id = 914;
        Self {
            base: base,
            text: String::new(),
            kerning: 0
        }
    }

    pub fn from(text: Option<String>, kerning: i32) -> Self {
        let mut base = GameObject::new();
        base.id = 914;
        Self {
            base: base,
            text: text.unwrap_or_default(),
            kerning: kerning
        }
    }

    /// Serializes a TextGameObject (save string)
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use gdfabric::gameobject::TextGameObject;
    /// let obj = TextGameObject::from(Some("Hello, World!".to_string()), 0);
    /// let serialized = obj.serialize();
    /// assert_eq!(serialized, "1,1,2,15,3,15,31,SGVsbG8sIFdvcmxkIQ==;");
    /// ```
    /// 
    /// # Returns
    /// 
    /// A string in the format "key,value,key,value,...;"
    pub fn serialize(&self) -> String {
        let mut ret: String = self.base.base_serialize();

        let b64_text = general_purpose::STANDARD.encode(&self.text);
        ret.push_str(format!("31,{},", b64_text).as_str());
        ret.push_str(&prop!(self.kerning, 488, 0));

        ret.pop();
        ret.push(';');
        ret
    }
}

impl GameObjectTrait for GameObject {
    fn serialize(&self) -> String {
        self.serialize()
    }
}

impl GameObjectTrait for TextGameObject {
    fn serialize(&self) -> String {
        self.serialize()
    }
}

macro_rules! display_game_object {
    ($struct_name:ident) => {
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.serialize())
            }
        }
    };
}

display_game_object!(GameObject);
display_game_object!(TextGameObject);