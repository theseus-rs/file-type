use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2541: FileType = FileType {
    file_format: &FileFormat {
        id: 2_541,
        source_type: SourceType::Pronom,
        name: "Persuasion Mac Document",
        extensions: &["pn4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x35]),
                        Token::WildcardCount(5),
                        Token::Literal(&[0x56, 0x88]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_540,
        }],
    },
};
