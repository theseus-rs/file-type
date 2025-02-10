use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2374: FileType = FileType {
    file_format: &FileFormat {
        id: 2_374,
        source_type: SourceType::Pronom,
        name: "Bentley Microstation Hidden Line File",
        extensions: &["hln"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x08, 0x09, 0xFE])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xFF])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_176,
        }],
    },
};
