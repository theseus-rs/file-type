use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1967: FileType = FileType {
    file_format: &FileFormat {
        id: 1_967,
        source_type: SourceType::Pronom,
        name: "Folio Infobase File",
        extensions: &["nfo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(1_292),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x6F, 0x6C, 0x69, 0x6F])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(1_292),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x76, 0x32, 0x30, 0x30])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
