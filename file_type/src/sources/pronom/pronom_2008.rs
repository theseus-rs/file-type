use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2008: FileType = FileType {
    file_format: &FileFormat {
        id: 2_008,
        source_type: SourceType::Pronom,
        name: "RData",
        extensions: &["rdata"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x44, 0x58, 0x32, 0x0A, 0x58, 0x0A])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::CanBeContainedBy,
            id: 386,
        }],
    },
};
