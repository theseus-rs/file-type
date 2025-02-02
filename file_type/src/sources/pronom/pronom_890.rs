use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_890: FileFormat = FileFormat {
    id: 890,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor 1-3 for DOS and 2 for Windows",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01, 0xFE]),
                    Token::WildcardCount(18),
                    Token::Any(&[&[Token::Literal(&[0xD0])], &[Token::Literal(&[0xC4])]]),
                    Token::Literal(&[0x02]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
