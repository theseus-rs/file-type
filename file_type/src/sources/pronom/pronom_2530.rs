use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2530: FileType = FileType {
    file_format: &FileFormat {
        id: 2_530,
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
                        0x03, 0x4A, 0x42, 0x4F, 0x4F, 0xF5, 0x3C, 0x55,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_529,
        }],
    },
};
