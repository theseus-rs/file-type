use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851744: FileFormat = FileFormat {
    id: 105_851_744,
    puid: "wikidata/105851744",
    name: "CodeView 4 Symbolic debug information",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x42, 0x30, 0x39])],
            },
        }],
    }],
    related_formats: &[],
};
