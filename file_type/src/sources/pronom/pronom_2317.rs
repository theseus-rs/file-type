use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2317: FileType = FileType {
    file_format: &FileFormat {
        id: 2_317,
        source_type: SourceType::Pronom,
        name: "QuarkXPress Project",
        extensions: &["qxp", "qpt", "qwd"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(2),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D, 0x4D, 0x58, 0x50, 0x52]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x50]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(2),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x49, 0x49, 0x58, 0x50, 0x52]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x50]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 255,
        }],
    },
};
