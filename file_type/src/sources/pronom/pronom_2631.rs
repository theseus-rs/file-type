use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2631: FileType = FileType {
    file_format: &FileFormat {
        id: 2_631,
        source_type: SourceType::Pronom,
        name: "Pentax PEF Image File",
        extensions: &["pef"],
        media_types: &["image/dng"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x50, 0x45, 0x4E, 0x54, 0x41, 0x58]),
                        Token::WildcardCountRange(0, 2_048),
                        Token::Literal(&[0x41, 0x4F, 0x43, 0x00, 0x4D, 0x4D]),
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
