use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851847: FileFormat = FileFormat {
    id: 105_851_847,
    puid: "wikidata/105851847",
    name: "SBStudio II sounds",
    extensions: &["sou"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
