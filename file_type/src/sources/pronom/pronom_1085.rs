use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1085: FileType = FileType {
    file_format: &FileFormat {
        id: 1_085,
        source_type: SourceType::Pronom,
        name: "Lotus WordPro Document",
        extensions: &["lwp"],
        media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x6F, 0x72, 0x64, 0x50, 0x72, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x4C, 0x57, 0x50, 0x37, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF,
                        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 504,
        }],
    },
};
