use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1485: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485,
        source_type: SourceType::Pronom,
        name: "Vectorworks",
        extensions: &["vwx"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x08, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94,
                            0x00, 0x20, 0x56, 0x57, 0x31, 0x35, 0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0x56, 0x57, 0x31, 0x35, 0x2E, 0x30]),
                        Token::WildcardCountRange(0, 29),
                        Token::Literal(&[
                            0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_238,
        }],
    },
};
