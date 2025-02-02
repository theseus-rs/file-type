use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2511: FileFormat = FileFormat {
    id: 2_511,
    source_type: SourceType::Pronom,
    name: "IntelliFont Font File",
    extensions: &["type", "lib"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x44, 0x00, 0x01, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
