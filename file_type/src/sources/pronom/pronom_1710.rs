use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1710: FileType = FileType {
    file_format: &FileFormat {
        id: 1_710,
        source_type: SourceType::Pronom,
        name: "Variant Call Format",
        extensions: &["vcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x66, 0x69, 0x6C, 0x65, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                        0x3D, 0x56, 0x43, 0x46, 0x76, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_711,
        }],
    },
};
