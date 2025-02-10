use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1530: FileType = FileType {
    file_format: &FileFormat {
        id: 1_530,
        source_type: SourceType::Pronom,
        name: "Bink Video Format",
        extensions: &["bik"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x49, 0x4B]),
                        Token::Any(&[
                            &[Token::Literal(&[0x62])],
                            &[Token::Literal(&[0x64])],
                            &[Token::Literal(&[0x66])],
                            &[Token::Literal(&[0x67])],
                            &[Token::Literal(&[0x68])],
                            &[Token::Literal(&[0x69])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_531,
        }],
    },
};
