use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_768: FileType = FileType {
    file_format: &FileFormat {
        id: 768,
        source_type: SourceType::Pronom,
        name: "DROID Signature File Format",
        extensions: &["xml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 50),
                        Token::Literal(&[
                            0x3C, 0x46, 0x46, 0x53, 0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65,
                            0x46, 0x69, 0x6C, 0x65, 0x20,
                        ]),
                        Token::WildcardCountRange(0, 100),
                        Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
