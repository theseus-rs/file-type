use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1083: FileType = FileType {
    file_format: &FileFormat {
        id: 1_083,
        source_type: SourceType::Pronom,
        name: "Interchange File Format Interleaved Bitmap",
        extensions: &["iff", "lbm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x49, 0x4C, 0x42, 0x4D, 0x42, 0x4D, 0x48, 0x44]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 221,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 221,
            },
        ],
    },
};
