use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856169: FileFormat = FileFormat {
    id: 105_856_169,
    puid: "wikidata/105856169",
    name: "Build engine Demo/replay data",
    extensions: &["dem"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x45, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
