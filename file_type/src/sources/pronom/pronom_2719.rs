use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2719: FileType = FileType {
    file_format: &FileFormat {
        id: 2_719,
        source_type: SourceType::Pronom,
        name: "SWiSH Movie File",
        extensions: &["swi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x4F, 0x62, 0x6A,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        }],
    },
};
