use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3889: FileType = FileType {
    file_format: &FileFormat {
        id: 3_889,
        source_type: SourceType::Pronom,
        name: "KryoFlux Stream",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(60),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x72, 0x79, 0x6F, 0x46, 0x6C, 0x75, 0x78, 0x20, 0x44, 0x69, 0x73,
                        0x6B, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x2C, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_454,
        }],
    },
};
