use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_727: FileType = FileType {
    file_format: &FileFormat {
        id: 727,
        source_type: SourceType::Pronom,
        name: "Windows Bitmap",
        extensions: &["ddb", "bmp"],
        media_types: &["image/bmp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x01]),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 728,
        }],
    },
};
