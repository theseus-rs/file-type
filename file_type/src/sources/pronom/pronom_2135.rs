use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2135: FileType = FileType {
    file_format: &FileFormat {
        id: 2_135,
        source_type: SourceType::Pronom,
        name: "QuarkXPress Document",
        extensions: &["qxd", "qxt", "qwd"],
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
                            Token::Any(&[
                                &[Token::Literal(&[0x39])],
                                &[Token::Literal(&[0x3A])],
                                &[Token::Literal(&[0x3B])],
                                &[Token::Literal(&[0x3D])],
                                &[Token::Literal(&[0x3E])],
                            ]),
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
                            Token::Literal(&[0x3E]),
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
