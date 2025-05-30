use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1531: FileType = FileType {
    file_format: &FileFormat {
        id: 1_531,
        source_type: SourceType::Pronom,
        name: "Bink Video Format",
        extensions: &["bik2", "bk2"],
        media_types: &["video/vnd.radgamettools.bink"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4B, 0x42, 0x32]),
                        Token::Any(&[
                            &[Token::Literal(&[0x61])],
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
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_530,
        }],
    },
};
