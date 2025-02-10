use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1366: FileType = FileType {
    file_format: &FileFormat {
        id: 1_366,
        source_type: SourceType::Pronom,
        name: "Image Cytometry Standard",
        extensions: &["ics"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x09, 0x0A, 0x69, 0x63, 0x73, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x09, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_365,
        }],
    },
};
