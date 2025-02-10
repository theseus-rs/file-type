use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2744: FileType = FileType {
    file_format: &FileFormat {
        id: 2_744,
        source_type: SourceType::Pronom,
        name: "Common Instrument File (CIF)",
        extensions: &["ci2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x43, 0xBD, 0x0A, 0x00, 0x01]),
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
                relationship_type: RelationshipType::EquivalentTo,
                id: 2_807,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_742,
            },
        ],
    },
};
