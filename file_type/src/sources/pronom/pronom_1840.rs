use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1840: FileFormat = FileFormat {
    id: 1_840,
    source_type: SourceType::Pronom,
    name: "Stata Data (DTA) Format",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x73]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::Literal(&[0x01]),
                    Token::WildcardCount(105),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
