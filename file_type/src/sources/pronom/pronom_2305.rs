use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2305: FileType = FileType {
    file_format: &FileFormat {
        id: 2_305,
        source_type: SourceType::Pronom,
        name: "Access Report Snapshot",
        extensions: &["snp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                        Token::WildcardCount(56),
                        Token::Literal(&[
                            0x5F, 0x41, 0x63, 0x63, 0x52, 0x70, 0x74, 0x5F, 0x2E, 0x73, 0x6E, 0x70,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 801,
        }],
    },
};
