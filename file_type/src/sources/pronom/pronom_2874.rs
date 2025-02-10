use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2874: FileType = FileType {
    file_format: &FileFormat {
        id: 2_874,
        source_type: SourceType::Pronom,
        name: "Husqvarna Embroidery Stitch File",
        extensions: &["hus"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[&[Token::Literal(&[0x5B])], &[Token::Literal(&[0x5D])]]),
                        Token::Any(&[
                            &[Token::Literal(&[0xAF])],
                            &[Token::Literal(&[0xFF])],
                            &[Token::Literal(&[0xFC])],
                        ]),
                        Token::Literal(&[0xC8, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_876,
        }],
    },
};
