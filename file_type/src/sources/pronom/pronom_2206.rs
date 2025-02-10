use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2206: FileType = FileType {
    file_format: &FileFormat {
        id: 2_206,
        source_type: SourceType::Pronom,
        name: "Muvee Reveal Project File",
        extensions: &["rvl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(40),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6D, 0x75, 0x76, 0x65, 0x65, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                        0x74, 0x3E, 0x3C, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
