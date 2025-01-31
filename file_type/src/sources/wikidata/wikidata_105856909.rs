use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856909: FileFormat = FileFormat {
    id: 105_856_909,
    puid: "wikidata/105856909",
    name: "MapSource GPS Waypoint Database",
    extensions: &["gdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x73, 0x52, 0x63, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
