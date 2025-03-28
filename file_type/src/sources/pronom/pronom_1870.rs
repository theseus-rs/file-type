use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1870: FileType = FileType {
    file_format: &FileFormat {
        id: 1_870,
        source_type: SourceType::Pronom,
        name: "Leaf Mosaic Raw Image",
        extensions: &["mos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                        Token::WildcardCountRange(4, 36_864),
                        Token::Literal(&[0x50, 0x4B, 0x54, 0x53, 0x00, 0x00, 0x00, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        }],
    },
};
