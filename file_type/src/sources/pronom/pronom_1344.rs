use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1344: FileType = FileType {
    file_format: &FileFormat {
        id: 1_344,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Macro",
        extensions: &["xlm"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x09, 0x02, 0x06, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x40, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 679,
        }],
    },
};
