use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1675: FileFormat = FileFormat {
    id: 2_511,
    puid: "fmt/1675",
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
