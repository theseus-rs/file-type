use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2733: FileFormat = FileFormat {
    id: 2_733,
    source_type: SourceType::Pronom,
    name: "vCard",
    extensions: &["vcf", "vcard"],
    media_types: &["text/vcard"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x43, 0x41, 0x52, 0x44,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x0A, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3A, 0x32, 0x2E, 0x31,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4E, 0x44, 0x3A, 0x56, 0x43, 0x41, 0x52, 0x44,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_143,
    }],
};
