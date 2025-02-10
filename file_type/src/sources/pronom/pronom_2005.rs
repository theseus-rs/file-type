use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2005: FileType = FileType {
    file_format: &FileFormat {
        id: 2_005,
        source_type: SourceType::Pronom,
        name: "ZModeler Z3D",
        extensions: &["z3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x4D, 0x33, 0x53, 0x03])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_004,
        }],
    },
};
