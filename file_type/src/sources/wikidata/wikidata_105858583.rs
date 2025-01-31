use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858583: FileFormat = FileFormat {
    id: 105_858_583,
    puid: "wikidata/105858583",
    name: "Gladius game data archive (PS2)",
    extensions: &["bec"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x63, 0x65, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
