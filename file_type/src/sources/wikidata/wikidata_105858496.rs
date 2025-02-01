use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858496: FileFormat = FileFormat {
    id: 105_858_496,
    puid: "wikidata/105858496",
    name: "AmigaBASIC source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF5, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
