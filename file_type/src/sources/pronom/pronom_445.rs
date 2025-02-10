use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_445: FileType = FileType {
    file_format: &FileFormat {
        id: 445,
        source_type: SourceType::Pronom,
        name: "CorelDraw Drawing",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46]),
                        Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x58])]]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x43, 0x44, 0x52, 0x38, 0x76, 0x72, 0x73, 0x6E]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_741,
        }],
    },
};
