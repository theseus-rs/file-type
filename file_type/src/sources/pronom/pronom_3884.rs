use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3884: FileType = FileType {
    file_format: &FileFormat {
        id: 3_884,
        source_type: SourceType::Pronom,
        name: "Visualization Toolkit",
        extensions: &["vtk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x76, 0x74, 0x6B, 0x20, 0x44, 0x61, 0x74, 0x61, 0x46, 0x69,
                        0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34,
                        0x2E, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 3_883,
        }],
    },
};
