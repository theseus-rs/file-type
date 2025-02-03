use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1148: FileFormat = FileFormat {
    id: 1_148,
    source_type: SourceType::Pronom,
    name: "Macromedia FreeHand MX",
    extensions: &["fh11"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x1C, 0x01, 0x00, 0x00, 0x02, 0x00, 0x04, 0x1C, 0x01, 0x14, 0x00, 0x02,
                            0x00, 0x14, 0x1C, 0x01, 0x16, 0x00, 0x02, 0x00, 0x09, 0x1C, 0x01, 0x1E,
                            0x00, 0x0A, 0x46, 0x72, 0x65, 0x65, 0x48, 0x61, 0x6E, 0x64, 0x31, 0x31,
                            0x1C, 0x01, 0x28, 0x00, 0x08,
                        ]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x30, 0x30, 0x1C, 0x01, 0x46, 0x00, 0x08, 0x32, 0x30]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x00, 0xF3, 0x00, 0xF0, 0x00, 0x86, 0x46, 0x6C, 0x61, 0x74, 0x65,
                        0x44, 0x65, 0x63, 0x6F, 0x64, 0x65, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x1C,
                        0x09, 0x0A, 0x00, 0x04, 0x00,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
