use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1791: FileType = FileType {
    file_format: &FileFormat {
        id: 1_791,
        source_type: SourceType::Pronom,
        name: "Extensible Metadata Platform Format",
        extensions: &["xmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x78, 0x3A, 0x78, 0x6D, 0x70, 0x6D, 0x65, 0x74, 0x61, 0x20, 0x78,
                            0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x78, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[
                            0x61, 0x64, 0x6F, 0x62, 0x65, 0x3A, 0x6E, 0x73, 0x3A, 0x6D, 0x65, 0x74,
                            0x61, 0x2F,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_358,
        }],
    },
};
