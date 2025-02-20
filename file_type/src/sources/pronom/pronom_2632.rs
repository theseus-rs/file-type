use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2632: FileType = FileType {
    file_format: &FileFormat {
        id: 2_632,
        source_type: SourceType::Pronom,
        name: "The Spectral Geologist Dataset",
        extensions: &["tsg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x53, 0x47, 0x20, 0x37, 0x2E, 0x31, 0x35, 0x20, 0x44, 0x61, 0x74,
                        0x61, 0x73, 0x65, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_633,
        }],
    },
};
