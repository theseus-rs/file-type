use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_621: FileType = FileType {
    file_format: &FileFormat {
        id: 621,
        source_type: SourceType::Pronom,
        name: "PCX",
        extensions: &["pcx", "pcc"],
        media_types: &["image/vnd.zbrush.pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 622,
        }],
    },
};
