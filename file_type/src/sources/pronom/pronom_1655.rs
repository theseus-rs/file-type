use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1655: FileType = FileType {
    file_format: &FileFormat {
        id: 1_655,
        source_type: SourceType::Pronom,
        name: "Personal Ancestral File (PAF)",
        extensions: &["paf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x46, 0x00, 0x33, 0x30, 0x30, 0x00, 0x33, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_656,
        }],
    },
};
