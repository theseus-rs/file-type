use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1388: FileType = FileType {
    file_format: &FileFormat {
        id: 1_388,
        source_type: SourceType::Pronom,
        name: "Apple Lossless Audio Codec",
        extensions: &["m4a", "mp4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(4),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x66, 0x74, 0x79, 0x70, 0x4D, 0x34, 0x41, 0x20,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x61, 0x6C, 0x61, 0x63]),
                            Token::WildcardCountRange(0, 72),
                            Token::Literal(&[0x61, 0x6C, 0x61, 0x63]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 924,
        }],
    },
};
