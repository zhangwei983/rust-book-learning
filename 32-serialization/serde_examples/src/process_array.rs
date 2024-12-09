use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::cmp;
use std::marker::PhantomData;

// Deserialize an array to find the maximum value without buffering all values into a Vec.
#[derive(Debug, Deserialize)]
struct Maxer {
    id: String,

    #[serde(deserialize_with = "deserialize_max")]
    #[serde(rename(deserialize = "values"))]
    max_value: u64,
}

fn deserialize_max<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a noneempty sequence of numbers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut max = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values in seq when looking for maximum"))?;

            while let Some(value) = seq.next_element()? {
                max = cmp::max(max, value);
            }

            Ok(max)
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let json_string = r#"
        {
          "id": "demo-deserialize-max",
          "values": [
            256,
            100,
            384,
            314,
            271
          ]
        }
    "#;

    let max: Maxer = serde_json::from_str(&json_string).unwrap();
    assert_eq!(max.id, "demo-deserialize-max");
    assert_eq!(max.max_value, 384);

    println!("Max deserialized: {:?}", max);

    println!("--- End module: {}", module_path!());
}
