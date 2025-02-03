use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2214: FileFormat = FileFormat {
    id: 2_214,
    source_type: SourceType::Pronom,
    name: "FinePrint",
    extensions: &["fp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x49, 0x4E]),
                    Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x45])]]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
