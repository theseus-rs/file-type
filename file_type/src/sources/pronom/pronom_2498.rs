use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2498: FileType = FileType {
    file_format: &FileFormat {
        id: 2_498,
        source_type: SourceType::Pronom,
        name: "Yamaha TX Wave Audio",
        extensions: &[
            "txw", "w01", "w02", "w03", "w04", "w05", "w06", "w07", "w08", "w09", "w10", "w11",
            "w12", "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22",
        ],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x4C, 0x4D, 0x38, 0x39, 0x35, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(5),
                        Token::Any(&[&[Token::Literal(&[0x49])], &[Token::Literal(&[0xC9])]]),
                        Token::Literal(&[0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_488,
        }],
    },
};
