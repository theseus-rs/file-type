use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1843: FileType = FileType {
    file_format: &FileFormat {
        id: 1_843,
        source_type: SourceType::Pronom,
        name: "Redcode RAW (R3D) Media File",
        extensions: &["r3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x45, 0x44, 0x32]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x52, 0x32]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_380,
        }],
    },
};
