use crate::pronom::regex::Regex;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

/// The position type for a byte sequence
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum PositionType {
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
pub enum Endianness {
    #[default]
    #[serde(rename = "Big-endian")]
    BigEndian,
    #[serde(rename = "Little-endian")]
    LittleEndian,
}

/// A byte sequence used to identify a file format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ByteSequence {
    #[serde(rename = "ByteSequenceID")]
    id: usize,
    position_type: PositionType,
    #[serde(deserialize_with = "deserialize_option_usize")]
    offset: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_usize")]
    max_offset: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_usize")]
    indirect_offset_location: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_usize")]
    indirect_offset_length: Option<usize>,
    #[serde(
        deserialize_with = "deserialize_endianness",
        serialize_with = "serialize_endianness"
    )]
    endianness: Option<Endianness>,
    #[serde(
        rename = "ByteSequenceValue",
        deserialize_with = "deserialize_regex",
        serialize_with = "serialize_regex"
    )]
    regex: Regex,
}

impl ByteSequence {
    /// Create a new byte sequence
    #[expect(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        id: usize,
        position_type: PositionType,
        offset: Option<usize>,
        max_offset: Option<usize>,
        indirect_offset_location: Option<usize>,
        indirect_offset_length: Option<usize>,
        endianness: Option<Endianness>,
        regex: Regex,
    ) -> Self {
        Self {
            id,
            position_type,
            offset,
            max_offset,
            indirect_offset_location,
            indirect_offset_length,
            endianness,
            regex,
        }
    }

    /// Get the ID of the byte sequence
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the position type of the byte sequence
    #[must_use]
    pub fn position_type(&self) -> &PositionType {
        &self.position_type
    }

    /// Get the offset of the byte sequence
    #[must_use]
    pub fn offset(&self) -> Option<usize> {
        self.offset
    }

    /// Get the maximum offset of the byte sequence
    #[must_use]
    pub fn max_offset(&self) -> Option<usize> {
        self.max_offset
    }

    /// Get the indirect offset location of the byte sequence
    #[must_use]
    pub fn indirect_offset_location(&self) -> Option<usize> {
        self.indirect_offset_location
    }

    /// Get the indirect offset length of the byte sequence
    #[must_use]
    pub fn indirect_offset_length(&self) -> Option<usize> {
        self.indirect_offset_length
    }

    /// Get the endianness of the byte sequence
    #[must_use]
    pub fn endianness(&self) -> Option<&Endianness> {
        self.endianness.as_ref()
    }

    /// Get the byte sequence regex
    #[must_use]
    pub fn regex(&self) -> &Regex {
        &self.regex
    }

    /// Check if the given data matches the byte sequence
    pub(crate) fn is_match(&self, bytes: &[u8]) -> bool {
        match self.position_type {
            PositionType::AbsoluteFromBOF => {
                let offset = self.offset.unwrap_or_default();
                self.regex.is_match_at(bytes, offset)
            }
            PositionType::AbsoluteFromEOF => {
                let offset = self.offset.unwrap_or_default();
                let offset = usize::checked_sub(bytes.len(), offset).unwrap_or(0);
                self.regex.is_match_at(bytes, offset)
            }
            PositionType::Variable => {
                for offset in self.offset.unwrap_or_default()..bytes.len() {
                    if self.regex.is_match_at(bytes, offset) {
                        return true;
                    }
                }
                false
            }
        }
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

#[expect(clippy::ref_option)]
fn serialize_endianness<S>(
    endianness: &Option<Endianness>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(endianness) = endianness {
        match endianness {
            Endianness::BigEndian => serializer.serialize_str("Big-endian"),
            Endianness::LittleEndian => serializer.serialize_str("Little-endian"),
        }
    } else {
        serializer.serialize_none()
    }
}

fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let regex = Regex::new(&value).map_err(Error::custom)?;
    Ok(regex)
}

fn serialize_regex<S>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let value = format!("{regex}");
    serializer.serialize_str(value.as_str())
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

        assert_eq!(byte_sequence.id(), 1955);
        assert!(matches!(
            byte_sequence.position_type(),
            PositionType::AbsoluteFromBOF
        ));
        assert_eq!(byte_sequence.offset(), Some(0));
        assert_eq!(byte_sequence.max_offset(), Some(0));
        assert!(byte_sequence.indirect_offset_location().is_none());
        assert!(byte_sequence.indirect_offset_length().is_none());
        let endianness = byte_sequence.endianness().expect("Endianness is None");
        assert!(matches!(endianness, Endianness::BigEndian));
        let regex = format!("{}", byte_sequence.regex());
        assert_eq!(regex, "7B*2269645F737472223A*22726574776565746564223A");
        Ok(())
    }

    #[test]
    fn test_new() {
        let regex =
            Regex::new("7B*2269645F737472223A*22726574776565746564223A").expect("Invalid regex");
        let byte_sequence = ByteSequence::new(
            1955,
            PositionType::AbsoluteFromBOF,
            Some(0),
            Some(0),
            None,
            None,
            Some(Endianness::BigEndian),
            regex,
        );
        assert_eq!(byte_sequence.id(), 1955);
        assert!(matches!(
            byte_sequence.position_type(),
            PositionType::AbsoluteFromBOF
        ));
        assert_eq!(byte_sequence.offset(), Some(0));
        assert_eq!(byte_sequence.max_offset(), Some(0));
        assert!(byte_sequence.indirect_offset_location().is_none());
        assert!(byte_sequence.indirect_offset_length().is_none());
        let endianness = byte_sequence.endianness().expect("Endianness is None");
        assert!(matches!(endianness, Endianness::BigEndian));
        let regex = format!("{}", byte_sequence.regex());
        assert_eq!(regex, "7B*2269645F737472223A*22726574776565746564223A");
    }
}
