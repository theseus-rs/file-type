use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1992: FileFormat = FileFormat {
    id: 1_992,
    source_type: SourceType::Pronom,
    name: "Blitz3D File Format",
    extensions: &["b3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x42, 0x33, 0x44]),
                    Token::WildcardCount(8),
                    Token::Any(&[
                        &[Token::Literal(&[0x54, 0x45, 0x58, 0x53])],
                        &[Token::Literal(&[0x42, 0x52, 0x55, 0x53])],
                        &[Token::Literal(&[0x4E, 0x4F, 0x44, 0x45])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
