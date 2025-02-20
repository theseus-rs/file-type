use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1895: FileType = FileType {
    file_format: &FileFormat {
        id: 1_895,
        source_type: SourceType::Pronom,
        name: "FAT Disk Image",
        extensions: &["img", "ima", "dsk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xEB]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x90]),
                        Token::WildcardCount(10),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x40])],
                            &[Token::Literal(&[0x80])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_436,
        }],
    },
};
