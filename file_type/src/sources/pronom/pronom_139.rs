use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_139: FileType = FileType {
    file_format: &FileFormat {
        id: 139,
        source_type: SourceType::Pronom,
        name: "Adobe Photoshop",
        extensions: &["psd", "pdd"],
        media_types: &["image/vnd.adobe.photoshop"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x38, 0x42, 0x50, 0x53, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_801,
        }],
    },
};
