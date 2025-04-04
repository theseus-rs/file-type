use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2436: FileType = FileType {
    file_format: &FileFormat {
        id: 2_436,
        source_type: SourceType::Pronom,
        name: "exFAT (Extensible File Allocation Table) Disc Image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0x76, 0x90, 0x45, 0x58, 0x46, 0x41, 0x54, 0x20, 0x20, 0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_895,
        }],
    },
};
