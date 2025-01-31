use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_439: FileFormat = FileFormat {
    id: 1_226,
    puid: "fmt/439",
    name: "BSDIFF",
    extensions: &["bsdiff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x53, 0x44, 0x49, 0x46, 0x46, 0x34, 0x30]),
                    Token::WildcardCount(24),
                    Token::Literal(&[0x42, 0x5A, 0x68]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x31, 0x41, 0x59, 0x26, 0x53, 0x59]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
