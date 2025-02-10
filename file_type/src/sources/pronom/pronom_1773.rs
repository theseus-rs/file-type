use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1773: FileType = FileType {
    file_format: &FileFormat {
        id: 1_773,
        source_type: SourceType::Pronom,
        name: "AppleSingle",
        extensions: &["as"],
        media_types: &["application/applefile"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x05, 0x16, 0x00, 0x00, 0x02, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_772,
        }],
    },
};
