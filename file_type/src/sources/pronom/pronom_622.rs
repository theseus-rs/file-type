use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_622: FileType = FileType {
    file_format: &FileFormat {
        id: 622,
        source_type: SourceType::Pronom,
        name: "PCX",
        extensions: &["pcx", "pcc"],
        media_types: &["image/vnd.zbrush.pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x02, 0x01])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 623,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 621,
            },
        ],
    },
};
