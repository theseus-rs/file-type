use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// The position type for a byte sequence
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum PositionType {
    #[default]
    #[serde(rename = "Absolute from BOF")]
    AbsoluteFromBOF,
    #[serde(rename = "Absolute from EOF")]
    AbsoluteFromEOF,
    #[serde(rename = "Variable")]
    Variable,
}

/// The endianness of a byte sequence
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum Endianness {
    #[default]
    #[serde(rename = "Big-endian")]
    BigEndian,
    #[serde(rename = "Little-endian")]
    LittleEndian,
}

/// A byte sequence used to identify a file format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct ByteSequence {
    #[serde(rename = "ByteSequenceID")]
    pub(crate) id: usize,
    pub(crate) position_type: PositionType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_usize"
    )]
    pub(crate) offset: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_usize"
    )]
    pub(crate) max_offset: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_usize"
    )]
    pub(crate) indirect_offset_location: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_usize"
    )]
    pub(crate) indirect_offset_length: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_endianness"
    )]
    pub(crate) endianness: Option<Endianness>,
    #[serde(rename = "ByteSequenceValue")]
    pub(crate) regex: String,
}

impl ByteSequence {
    /// Convert to the type used at runtime
    pub fn to_runtime_type(&self) -> file_type::Result<file_type::format::ByteSequence> {
        let position_type = match self.position_type {
            PositionType::AbsoluteFromBOF => file_type::format::PositionType::BOF,
            PositionType::AbsoluteFromEOF => file_type::format::PositionType::EOF,
            PositionType::Variable => file_type::format::PositionType::Variable,
        };
        let regex = file_type::format::Regex::new(&self.regex)?;
        let byte_sequence = file_type::format::ByteSequence {
            position_type,
            offset: self.offset,
            regex,
        };
        Ok(byte_sequence)
    }
}

fn deserialize_option_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty() {
        Ok(None)
    } else {
        let value: usize = value.parse().map_err(Error::custom)?;
        Ok(Some(value))
    }
}

fn deserialize_endianness<'de, D>(deserializer: D) -> Result<Option<Endianness>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty() {
        Ok(None)
    } else {
        match value.as_str() {
            "Big-endian" => Ok(Some(Endianness::BigEndian)),
            "Little-endian" => Ok(Some(Endianness::LittleEndian)),
            _ => Err(Error::custom("Invalid endianness")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let xml = indoc! {r"
          <ByteSequence>
            <ByteSequenceID>1955</ByteSequenceID>
            <PositionType>Absolute from BOF</PositionType>
            <Offset>0</Offset>
            <MaxOffset>0</MaxOffset>
            <IndirectOffsetLocation>
            </IndirectOffsetLocation>
            <IndirectOffsetLength>
            </IndirectOffsetLength>
            <Endianness>Big-endian</Endianness>
            <ByteSequenceValue>7B*2269645F737472223A*22726574776565746564223A</ByteSequenceValue>
          </ByteSequence>
        "};
        let byte_sequence: ByteSequence = from_str(xml)?;

        // Test serialization
        let xml = to_string(&byte_sequence)?;
        let byte_sequence: ByteSequence = from_str(xml.as_str())?;

        assert_eq!(byte_sequence.id, 1955);
        assert!(matches!(
            byte_sequence.position_type,
            PositionType::AbsoluteFromBOF
        ));
        assert_eq!(byte_sequence.offset, Some(0));
        assert_eq!(byte_sequence.max_offset, Some(0));
        assert!(byte_sequence.indirect_offset_location.is_none());
        assert!(byte_sequence.indirect_offset_length.is_none());
        let endianness = byte_sequence.endianness.expect("Endianness is None");
        assert!(matches!(endianness, Endianness::BigEndian));
        assert_eq!(
            byte_sequence.regex,
            "7B*2269645F737472223A*22726574776565746564223A"
        );
        Ok(())
    }
}
