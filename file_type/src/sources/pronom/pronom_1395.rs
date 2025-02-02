use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1395: FileFormat = FileFormat {
    id: 1_395,
    source_type: SourceType::Pronom,
    name: "Statistical Analysis System Data XPT (Windows)",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(92),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x58, 0x50, 0x5F, 0x50, 0x52, 0x4F]),
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x53, 0x41, 0x53, 0x39, 0x2E, 0x31]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6E, 0x64, 0x20, 0x6F, 0x66, 0x20, 0x44, 0x61, 0x74, 0x61,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
