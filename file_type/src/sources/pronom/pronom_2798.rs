use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2798: FileType = FileType {
    file_format: &FileFormat {
        id: 2_798,
        source_type: SourceType::Pronom,
        name: "Amazon Kindle eBook File",
        extensions: &["azw", "azw3", "mobi", "amr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(60),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
                            ])],
                            &[Token::Literal(&[
                                0x54, 0x45, 0x58, 0x74, 0x52, 0x45, 0x41, 0x64,
                            ])],
                        ]),
                        Token::WildcardCountRange(450, 15_000),
                        Token::Literal(&[0x6B, 0x69, 0x6E, 0x64, 0x6C, 0x65, 0x3A]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_144,
        }],
    },
};
