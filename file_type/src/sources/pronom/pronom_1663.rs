use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1663: FileFormat = FileFormat {
    id: 1_663,
    source_type: SourceType::Pronom,
    name: "Maya Binary File Format",
    extensions: &["mb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x34]),
                    Token::WildcardCount(4),
                    Token::Any(&[
                        &[Token::Literal(&[0x4D, 0x61, 0x79, 0x61])],
                        &[Token::Literal(&[0x4D, 0x41, 0x59, 0x41])],
                    ]),
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x34]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
