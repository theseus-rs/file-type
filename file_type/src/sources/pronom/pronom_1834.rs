use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1834: FileFormat = FileFormat {
    id: 1_834,
    source_type: SourceType::Pronom,
    name: "Stata Data (DTA) Format",
    extensions: &["dta"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x68]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::Literal(&[0x01, 0x00]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
