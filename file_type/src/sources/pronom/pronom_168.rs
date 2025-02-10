use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_168: FileType = FileType {
    file_format: &FileFormat {
        id: 168,
        source_type: SourceType::Pronom,
        name: "Lotus 1-2-3 Worksheet",
        extensions: &["wk4"],
        media_types: &["application/lotus123", "application/vnd.lotus-1-2-3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x1A, 0x00, 0x02, 0x10, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 166,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 167,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 169,
            },
        ],
    },
};
