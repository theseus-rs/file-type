use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1937: FileType = FileType {
    file_format: &FileFormat {
        id: 1_937,
        source_type: SourceType::Pronom,
        name: "Sony ARW RAW Image File",
        extensions: &["arw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCountRange(0, 512),
                        Token::Literal(&[0x53, 0x4F, 0x4E, 0x59]),
                        Token::WildcardCountRange(0, 6_144),
                        Token::Literal(&[0x00, 0xB0, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x03]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        }],
    },
};
