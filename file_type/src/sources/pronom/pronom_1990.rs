use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1990: FileType = FileType {
    file_format: &FileFormat {
        id: 1_990,
        source_type: SourceType::Pronom,
        name: "Cinema 4D",
        extensions: &["c4d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x34, 0x44, 0x43, 0x34, 0x44, 0x36])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_327,
        }],
    },
};
