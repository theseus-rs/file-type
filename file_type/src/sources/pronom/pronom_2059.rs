use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2059: FileType = FileType {
    file_format: &FileFormat {
        id: 2_059,
        source_type: SourceType::Pronom,
        name: "FO File",
        extensions: &["fo"],
        media_types: &["\tapplication/vnd.software602.filler.form+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x6F, 0x3A, 0x72, 0x6F, 0x6F, 0x74])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
