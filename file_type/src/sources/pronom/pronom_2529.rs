use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2529: FileType = FileType {
    file_format: &FileFormat {
        id: 2_529,
        source_type: SourceType::Pronom,
        name: "Asymetrix Compel Presentation",
        extensions: &["cpl", "art"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x03, 0x4A, 0x42, 0x4F, 0x4E, 0xD3, 0x2C, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_530,
        }],
    },
};
