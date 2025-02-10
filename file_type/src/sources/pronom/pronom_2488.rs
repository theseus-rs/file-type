use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2488: FileType = FileType {
    file_format: &FileFormat {
        id: 2_488,
        source_type: SourceType::Pronom,
        name: "Yamaha Wave Audio",
        extensions: &["s01", "u01", "f01", "w01"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4D, 0x38, 0x39, 0x35, 0x33, 0x00])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_498,
        }],
    },
};
