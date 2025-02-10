use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1257: FileType = FileType {
    file_format: &FileFormat {
        id: 1_257,
        source_type: SourceType::Pronom,
        name: "Asymetrix Toolbook File",
        extensions: &["tbk", "sbk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x03, 0x4A, 0x42, 0x4F]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0x14]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_645,
        }],
    },
};
