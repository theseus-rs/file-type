use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_455: FileFormat = FileFormat {
    id: 1_242,
    puid: "fmt/455",
    name: "Verity Collection Index Pending Transaction File",
    extensions: &["trn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x4C, 0x41, 0x53, 0x54, 0x20, 0x4C, 0x4F, 0x47, 0x43, 0x48, 0x45, 0x43,
                        0x4B, 0x20, 0x22,
                    ]),
                    Token::WildcardCount(24),
                    Token::Literal(&[
                        0x22, 0x0A, 0x4C, 0x41, 0x53, 0x54, 0x20, 0x43, 0x4C, 0x45, 0x41, 0x4E,
                        0x20,
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x22]),
                    Token::WildcardCount(24),
                    Token::Literal(&[
                        0x22, 0x0A, 0x4C, 0x41, 0x53, 0x54, 0x20, 0x4F, 0x50, 0x54, 0x49, 0x4D,
                        0x49, 0x5A, 0x45, 0x20,
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x22]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x22, 0x0A]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
