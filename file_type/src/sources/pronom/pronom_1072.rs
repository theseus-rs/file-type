use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1072: FileFormat = FileFormat {
    id: 1_072,
    source_type: SourceType::Pronom,
    name: "EndNote Filter File",
    extensions: &["enf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x45, 0x4E, 0x44, 0x4E, 0x45, 0x4C, 0x32, 0x73, 0x00, 0x00, 0x00,
                    ]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x26, 0x00, 0x03]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
