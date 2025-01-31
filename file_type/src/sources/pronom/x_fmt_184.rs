use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_184: FileFormat = FileFormat {
    id: 257,
    puid: "x-fmt/184",
    name: "Sun Raster Image",
    extensions: &["ras", "sun"],
    media_types: &["image/x-sun-raster"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x59, 0xA6, 0x6A, 0x95]),
                    Token::WildcardCount(8),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::SingleWildcard,
                ],
            },
        }],
    }],
    related_formats: &[],
};
