use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1607: FileType = FileType {
    file_format: &FileFormat {
        id: 1_607,
        source_type: SourceType::Pronom,
        name: "HDF5",
        extensions: &["h5", "hdf", "hdf5", "nc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x48, 0x44, 0x46, 0x0D, 0x0A, 0x1A, 0x0A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_875,
        }],
    },
};
