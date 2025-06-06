use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_215: FileType = FileType {
    file_format: &FileFormat {
        id: 215,
        source_type: SourceType::Pronom,
        name: "Microsoft Windows Enhanced Metafile",
        extensions: &["emf"],
        media_types: &["image/emf"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x01, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(32),
                            Token::Literal(&[0x20, 0x45, 0x4D, 0x46, 0x00, 0x00, 0x01, 0x00]),
                            Token::WildcardCount(16),
                            Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x01, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x20, 0x45, 0x4D, 0x46, 0x00, 0x00, 0x01, 0x00]),
                            Token::WildcardCount(16),
                            Token::Literal(&[0x58, 0x00, 0x00, 0x00]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_089,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_090,
            },
        ],
    },
};
