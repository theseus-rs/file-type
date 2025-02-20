use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1635: FileType = FileType {
    file_format: &FileFormat {
        id: 1_635,
        source_type: SourceType::Pronom,
        name: "Quattro Pro Spreadsheet for Windows",
        extensions: &["wb1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x01, 0x10])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_636,
        }],
    },
};
