use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1758: FileType = FileType {
    file_format: &FileFormat {
        id: 1_758,
        source_type: SourceType::Pronom,
        name: "True Audio",
        extensions: &["tta"],
        media_types: &["audio/tta"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x54, 0x41, 0x32])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_757,
        }],
    },
};
