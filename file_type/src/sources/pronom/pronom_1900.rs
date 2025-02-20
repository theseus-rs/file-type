use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1900: FileType = FileType {
    file_format: &FileFormat {
        id: 1_900,
        source_type: SourceType::Pronom,
        name: "Alias Pix Image File",
        extensions: &["pix", "ico"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x18]),
                        Token::NotLiteral(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 802,
        }],
    },
};
