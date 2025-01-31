use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_191: FileFormat = FileFormat {
    id: 264,
    puid: "x-fmt/191",
    name: "AMI Professional Document",
    extensions: &["sam"],
    media_types: &["application/vnd.lotus-wordpro"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x5B, 0x76, 0x65, 0x72, 0x5D]),
                    Token::WildcardCount(6),
                    Token::Literal(&[0x5B, 0x73, 0x74, 0x79, 0x5D]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x5B, 0x65, 0x64, 0x6F, 0x63, 0x5D]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
