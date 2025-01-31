use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1887: FileFormat = FileFormat {
    id: 2_742,
    puid: "fmt/1887",
    name: "Common Instrument File (CIF)",
    extensions: &["ci1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0xBD, 0x0A, 0x00, 0x01]),
                    Token::WildcardCount(15),
                    Token::Literal(&[0x76, 0x31, 0x2E, 0x30, 0x61, 0x00]),
                    Token::WildcardCount(10),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_805,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 2_744,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
