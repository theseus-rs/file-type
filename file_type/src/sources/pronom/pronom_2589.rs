use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2589: FileType = FileType {
    file_format: &FileFormat {
        id: 2_589,
        source_type: SourceType::Pronom,
        name: "Nero Burning ROM Image File",
        extensions: &["nrg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4E, 0x45, 0x52]),
                        Token::Any(&[&[Token::Literal(&[0x35])], &[Token::Literal(&[0x4F])]]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_255,
        }],
    },
};
