use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1657: FileType = FileType {
    file_format: &FileFormat {
        id: 1_657,
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
                        0x35, 0x30, 0x30, 0x00, 0x35, 0x30, 0x30, 0x00, 0x50, 0x41, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_656,
        }],
    },
};
