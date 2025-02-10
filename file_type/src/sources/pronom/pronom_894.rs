use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_894: FileType = FileType {
    file_format: &FileFormat {
        id: 894,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0xFF])]]),
                            Token::Literal(&[0x00, 0x02, 0x00, 0x04, 0x04, 0x05, 0x54, 0x02]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x01, 0x00, 0x00, 0x00])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 169,
        }],
    },
};
