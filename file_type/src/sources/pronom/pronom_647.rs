use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_647: FileType = FileType {
    file_format: &FileFormat {
        id: 647,
        source_type: SourceType::Pronom,
        name: "Macromedia Flash",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x02])],
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
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 687,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 648,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 646,
            },
        ],
    },
};
