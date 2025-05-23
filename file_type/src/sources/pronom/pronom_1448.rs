use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1448: FileType = FileType {
    file_format: &FileFormat {
        id: 1_448,
        source_type: SourceType::Pronom,
        name: "MPEG-1 Elementary Stream",
        extensions: &["mpg", "mpeg", "m1v"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                        Token::WildcardCountRange(8, 136),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB8]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_439,
        }],
    },
};
