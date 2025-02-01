use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_226: FileFormat = FileFormat {
    id: 318,
    puid: "x-fmt/226",
    name: "ESRI Arc/Info Export File",
    extensions: &[
        "e00", "x00", "e01", "e02", "e03", "e04", "e05", "e06", "e07", "e08", "e09", "e10", "e11",
        "e12", "e13", "e14", "e15", "e16", "e17", "e18", "e19", "e20",
    ],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x45, 0x58, 0x50, 0x20, 0x20]),
                        Token::Range(&[0x30], &[0x31]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x45, 0x4F, 0x49]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x45, 0x4F, 0x53]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x0A]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
