use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2124: FileFormat = FileFormat {
    id: 2_124,
    source_type: SourceType::Pronom,
    name: "LocoScript Document",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4A, 0x4F, 0x59, 0x01]),
                    Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
