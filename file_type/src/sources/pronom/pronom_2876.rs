use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2876: FileType = FileType {
    file_format: &FileFormat {
        id: 2_876,
        source_type: SourceType::Pronom,
        name: "Husqvarna / Pfaff Embroidery Stitch File",
        extensions: &["vip"],
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
                        Token::Literal(&[0x90, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_874,
        }],
    },
};
