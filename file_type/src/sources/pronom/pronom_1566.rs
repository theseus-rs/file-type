use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1566: FileType = FileType {
    file_format: &FileFormat {
        id: 1_566,
        source_type: SourceType::Pronom,
        name: "Adobe Flash",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x15])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x00, 0x00])],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x43, 0x57, 0x53, 0x15])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x5A, 0x57, 0x53, 0x15])],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_567,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_565,
            },
        ],
    },
};
