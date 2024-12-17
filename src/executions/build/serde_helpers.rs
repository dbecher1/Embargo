use serde::{de::{self, Visitor}, Deserializer, Serializer};

pub fn serialize_modified<S>(modified: &u64, s: S) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    let mod_str = format!("{:X}", modified);
    s.serialize_str(&mod_str)
}

pub fn deserialize_modified<'de, D>(d: D) -> Result<u64, D::Error>
where 
    D: Deserializer<'de>
{
    struct ModifiedVisitor;

    impl<'de> Visitor<'de> for ModifiedVisitor {
        type Value = u64;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a hex value that maps to a u64")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where 
            E: de::Error {
                return match u64::from_str_radix(value, 16) {
                    Ok(n) => Ok(n),
                    Err(_) => Err(E::custom(format!("Input must be a hex string!")))
                }
                
            }
    }
    d.deserialize_any(ModifiedVisitor)
}
