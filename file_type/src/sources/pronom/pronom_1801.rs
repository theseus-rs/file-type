use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1801: FileType = FileType {
    file_format: &FileFormat {
        id: 1_801,
        source_type: SourceType::Pronom,
        name: "Adobe Photoshop Large Document Format",
        extensions: &["psb"],
        media_types: &["image/vnd.adobe.photoshop"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x38, 0x42, 0x50, 0x53, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 139,
        }],
    },
};
