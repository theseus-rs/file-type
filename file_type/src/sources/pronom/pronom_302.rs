use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_302: FileType = FileType {
    file_format: &FileFormat {
        id: 302,
        source_type: SourceType::Pronom,
        name: "Microsoft Paint",
        extensions: &["msp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x44, 0x61, 0x6E, 0x4D]),
                        Token::WildcardCount(22),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_717,
        }],
    },
};
