use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1512: FileType = FileType {
    file_format: &FileFormat {
        id: 1_512,
        source_type: SourceType::Pronom,
        name: "RF64 Multichannel Broadcast Wave format",
        extensions: &["wav", "rf64"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x46, 0x36, 0x34]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x64, 0x73, 0x36, 0x34]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x62, 0x65, 0x78, 0x74]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_511,
        }],
    },
};
