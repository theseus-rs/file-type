use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1053: FileFormat = FileFormat {
    id: 1_053,
    source_type: SourceType::Pronom,
    name: "Quicken Data Format",
    extensions: &["qdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xAC, 0x9E, 0xBD, 0x8F, 0x00, 0x00]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
