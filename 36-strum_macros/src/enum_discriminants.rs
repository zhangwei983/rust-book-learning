use std::str::FromStr;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, EnumString, IntoEnumIterator};

#[derive(Debug)]
struct NoDefault;

#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, EnumMessage))]
enum MyEnum {
    #[strum_discriminants(strum(message = "Variant 0"))]
    Variant0(NoDefault),
    Variant1 {
        a: NoDefault,
    },
}

#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(MyVariants))]
#[strum_discriminants(vis(pub))] // Visible outside the module.
enum MyEnum1 {
    Variant0(bool),
    Variant1 { a: bool },
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let my_discriminant_enum = MyEnumDiscriminants::from_str("Variant0").unwrap();
    println!("My discriminant enum: {:?}", my_discriminant_enum);

    let my_enum_variant = MyEnumDiscriminants::from(MyEnum::Variant0(NoDefault));
    println!("My discriminant variant: {:?}", my_enum_variant);
    println!(
        "My discriminant variant message: {:?}",
        my_enum_variant.get_message().unwrap()
    );

    let variants = MyVariants::iter().collect::<Vec<_>>();
    println!("My variants: {:?}", variants);

    println!("--- End module: {}", module_path!());
}
