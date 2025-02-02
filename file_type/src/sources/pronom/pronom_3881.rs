use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_3881: FileFormat = FileFormat {
    id: 3_881,
    source_type: SourceType::Pronom,
    name: "Protein Data Bank File",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x45, 0x52]),
                        Token::WildcardCountRange(74, 80),
                        Token::Literal(&[0x0A, 0x54, 0x49, 0x54, 0x4C, 0x45]),
                        Token::WildcardCountRange(74, 250),
                        Token::Literal(&[0x0A, 0x43, 0x4F, 0x4D, 0x50, 0x4E, 0x44]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x53, 0x4F, 0x55, 0x52, 0x43, 0x45]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x4B, 0x45, 0x59, 0x57, 0x44, 0x53]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x45, 0x58, 0x50, 0x44, 0x54, 0x41]),
                        Token::WildcardCountRange(74, 76),
                        Token::Literal(&[0x0A, 0x41, 0x55, 0x54, 0x48, 0x4F, 0x52]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x52, 0x45, 0x56, 0x44, 0x41, 0x54]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x52, 0x45, 0x4D, 0x41, 0x52, 0x4B]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x43, 0x52, 0x59, 0x53, 0x54, 0x31]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x4F, 0x52, 0x49, 0x47, 0x58, 0x31]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x0A, 0x53, 0x43, 0x41, 0x4C, 0x45, 0x31]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x0A, 0x4D, 0x41, 0x53, 0x54, 0x45, 0x52]),
                        Token::WildcardCountRange(74, 75),
                        Token::Literal(&[0x0A, 0x45, 0x4E, 0x44]),
                        Token::WildcardCountRange(77, 78),
                        Token::Literal(&[0x0A]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
