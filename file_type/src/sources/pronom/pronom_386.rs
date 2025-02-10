use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_386: FileType = FileType {
    file_format: &FileFormat {
        id: 386,
        source_type: SourceType::Pronom,
        name: "GZIP Format",
        extensions: &["gz", "z"],
        media_types: &["application/gzip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x8B, 0x08])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 2_008,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 2_009,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 2_029,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 2_286,
            },
        ],
    },
};
