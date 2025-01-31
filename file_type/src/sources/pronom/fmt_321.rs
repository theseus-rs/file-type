use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_321: FileFormat = FileFormat {
    id: 1_066,
    puid: "fmt/321",
    name: "ESRI Shapefile Header Index",
    extensions: &["aih"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x01]),
                    Token::WildcardCount(35),
                    Token::Literal(&[0x2E]),
                    Token::WildcardCount(34),
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
