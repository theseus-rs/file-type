use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_264: FileFormat = FileFormat {
    id: 384,
    puid: "x-fmt/264",
    name: "RAR Archive",
    extensions: &["rar"],
    media_types: &["application/vnd.rar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x73]),
                    Token::WildcardCount(34),
                    Token::Literal(&[0x14]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
