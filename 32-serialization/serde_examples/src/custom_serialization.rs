use serde::{
    ser::{SerializeStruct, SerializeStructVariant, SerializeTupleVariant},
    Deserialize, Serialize, Serializer,
};

#[derive(Debug, Deserialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Write our own #[derive(Serialize)] for a struct.
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

#[derive(Debug, Deserialize)]
enum Example {
    Color { r: u8, g: u8, b: u8 },

    Point2D(f64, f64),

    Inches(u64),

    Instance,
}

// Write our own #[derive(Serialize)] for an Enum.
impl Serialize for Example {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Example::Color { r, g, b } => {
                let mut state = serializer.serialize_struct_variant("Example", 0, "Color", 3)?;
                state.serialize_field("r", &r)?;
                state.serialize_field("g", &g)?;
                state.serialize_field("b", &b)?;
                state.end()
            }
            Example::Point2D(x, y) => {
                let mut state = serializer.serialize_tuple_variant("Example", 1, "Point2D", 2)?;
                state.serialize_field(&x)?;
                state.serialize_field(&y)?;
                state.end()
            }
            Example::Inches(value) => {
                serializer.serialize_newtype_variant("Example", 2, "Inches", &value)
            }
            Example::Instance => serializer.serialize_unit_variant("Example", 3, "Instance"),
        }
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Test Color struct.
    let color = Color { r: 255, g: 0, b: 0 };
    let color_json = serde_json::to_string(&color).unwrap();
    println!("Color json: {}", color_json);

    let color_deserialized: Color = serde_json::from_str(&color_json).unwrap();
    println!("Color: {:?}", color_deserialized);

    // Test Color enum.
    let color_enum = Example::Color { r: 255, g: 0, b: 0 };
    let color_enum_json = serde_json::to_string(&color_enum).unwrap();
    println!("Color enum json: {}", color_enum_json);

    let color_enum_deserialized: Example = serde_json::from_str(&color_enum_json).unwrap();
    println!("Color enum: {:?}", color_enum_deserialized);

    // Test Point2D enum.
    let point_enum = Example::Point2D(30f64, 30f64);
    let point_enum_json = serde_json::to_string(&point_enum).unwrap();
    println!("Point2D enum json: {}", point_enum_json);

    let point_enum_deserialized: Example = serde_json::from_str(&point_enum_json).unwrap();
    println!("Point2D enum: {:?}", point_enum_deserialized);

    println!("--- End module: {}", module_path!());
}
