use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2573: FileType = FileType {
    file_format: &FileFormat {
        id: 2_573,
        source_type: SourceType::Pronom,
        name: "Esri Shapefile Geospatial Metadata File",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x3C, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x78,
                                0x6D, 0x6C, 0x3A, 0x6C, 0x61, 0x6E, 0x67, 0x3D, 0x22,
                            ]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x22, 0x3E]),
                            Token::WildcardCountRange(0, 2),
                            Token::Literal(&[0x3C, 0x45, 0x73, 0x72, 0x69, 0x3E]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x2F, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x3E,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
