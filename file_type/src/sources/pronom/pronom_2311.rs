use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2311: FileFormat = FileFormat {
    id: 2_311,
    source_type: SourceType::Pronom,
    name: "Phantom CINE Video File",
    extensions: &["cine", "cin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x49]),
                    Token::WildcardCount(2),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                    Token::WildcardCount(1),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
