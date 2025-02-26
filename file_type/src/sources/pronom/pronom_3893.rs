use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3893: FileType = FileType {
    file_format: &FileFormat {
        id: 3_893,
        source_type: SourceType::Pronom,
        name: "askSam Document for DOS",
        extensions: &["ask"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x73, 0x6B, 0x53, 0x61, 0x6D, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 3_894,
        }],
    },
};
