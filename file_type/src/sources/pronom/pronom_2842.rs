use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2842: FileType = FileType {
    file_format: &FileFormat {
        id: 2_842,
        source_type: SourceType::Pronom,
        name: "ICC Profile",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x02]),
                        Token::WildcardCount(27),
                        Token::Literal(&[0x61, 0x63, 0x73, 0x70]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_843,
        }],
    },
};
