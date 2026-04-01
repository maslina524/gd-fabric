use core::fmt;
use std::collections::HashSet;
use base64::{Engine as _, engine::general_purpose};

#[derive(Clone)]
pub enum SingleColorType {
    Default = 0,
    Base = 1,
    Detail = 2
}

#[derive(Clone)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
    s_checked: bool,
    v_checked: bool,
}
impl Hsv {
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

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
    // Base
    pub id: u16,
    pub pos: Point,
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
    pub hsv_1: Hsv,
    pub hsv_2: Hsv,
    pub groups: HashSet<u16>,
    pub p_groups: HashSet<u16>,
    // TextObject
    pub text: String,
    pub kerning: i32,
}

impl GameObject {
    pub fn new() -> Self {
        Self {
            // Base
            id: 1,
            pos: Point { x: 15., y: 15. },
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
            hsv_1: Hsv::new(),
            hsv_2: Hsv::new(),
            groups: HashSet::new(),
            p_groups: HashSet::new(),
            // TExtObject
            text: String::new(),
            kerning: 0,
        }
    }

    pub fn from(id: u16, pos: Point) -> Self {
        let mut ret = Self::new();
        ret.id = id;
        ret.pos = pos;
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
        let mut ret = String::new();

        // Base
        ret.push_str(format!("1,{},2,{},3,{},", self.id, self.pos.x, self.pos.y).as_str());
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

        let hsv_def = Hsv::new().serialize();
        if self.hsv_1.serialize() != hsv_def { ret.push_str("41,1,"); }
        ret.push_str(&prop!(self.hsv_1.serialize(), 43, hsv_def));
        if self.hsv_2.serialize() != hsv_def { ret.push_str("42,1,"); }
        ret.push_str(&prop!(self.hsv_2.serialize(), 44, hsv_def));

        // GROUPS
        let mut all_groups = self.groups.clone();
        all_groups.extend(&self.p_groups);
        
        if !all_groups.is_empty() {
            let all_groups_string = all_groups.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(".");
            ret.push_str(format!("57,{all_groups_string},").as_str());
        }

        // PARENT GROUPS
        if !self.p_groups.is_empty() {
            let p_groups_string = self.p_groups.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(".");
            ret.push_str(format!("274,{p_groups_string},").as_str());
        }

        // TextObject
        let b64 = &prop!(general_purpose::STANDARD.encode(&self.text), 31, String::new());
        ret.push_str(b64.as_str());
        ret.push_str(&prop!(self.kerning, 488, 0));

        ret.pop();
        ret.push(';');
        ret
    }
}

pub struct TextGameObject {}

impl TextGameObject {
    pub fn new() -> GameObject {
        let mut base = GameObject::new();
        base.id = 914;

        base
    }

    pub fn from(text: String, kerning: i32) -> GameObject {
        let mut base = GameObject::new();
        base.id = 914;
        base.text = text;
        base.kerning = kerning;

        base
    }
}

impl fmt::Display for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize())
    }
}