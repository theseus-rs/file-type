use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851496: FileFormat = FileFormat {
    id: 105_851_496,
    puid: "wikidata/105851496",
    name: "TECkit compiled mapping",
    extensions: &["tec"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x51, 0x6D, 0x70, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
