use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2819: FileType = FileType {
    file_format: &FileFormat {
        id: 2_819,
        source_type: SourceType::Pronom,
        name: "Zoom Project Settings",
        extensions: &["hprj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x4F, 0x4F, 0x4D, 0x20, 0x48, 0x36, 0x20, 0x70, 0x72, 0x6A, 0x65,
                        0x63, 0x74, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_817,
        }],
    },
};
