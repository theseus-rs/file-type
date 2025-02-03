use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2127: FileFormat = FileFormat {
    id: 2_127,
    source_type: SourceType::Pronom,
    name: "LocoScript Professional",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x4F, 0x43, 0x01]),
                    Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x03])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
