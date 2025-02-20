use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1985: FileType = FileType {
    file_format: &FileFormat {
        id: 1_985,
        source_type: SourceType::Pronom,
        name: "Alias Studio Wire File",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(12),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x6C, 0x69, 0x61, 0x73, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F,
                        ]),
                        Token::WildcardCount(13),
                        Token::Literal(&[0x39, 0x2E, 0x30]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_980,
        }],
    },
};
