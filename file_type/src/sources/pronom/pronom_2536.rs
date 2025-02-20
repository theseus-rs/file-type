use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2536: FileType = FileType {
    file_format: &FileFormat {
        id: 2_536,
        source_type: SourceType::Pronom,
        name: "OGC GeoPackage",
        extensions: &["gpkg"],
        media_types: &["application/geopackage+sqlite3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                            0x74, 0x20, 0x33,
                        ]),
                        Token::WildcardCount(53),
                        Token::Literal(&[0x47, 0x50]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x67, 0x70, 0x6B, 0x67, 0x5F, 0x73, 0x70, 0x61, 0x74, 0x69, 0x61, 0x6C,
                            0x5F, 0x72, 0x65, 0x66, 0x5F, 0x73, 0x79, 0x73,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_528,
        }],
    },
};
