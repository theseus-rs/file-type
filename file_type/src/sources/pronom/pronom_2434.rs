use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2434: FileType = FileType {
    file_format: &FileFormat {
        id: 2_434,
        source_type: SourceType::Pronom,
        name: "True Colour Picture [Spooky Sprites] (Atari Falcon)",
        extensions: &["trp", "tru"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x72, 0x75, 0x3F])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 2_435,
        }],
    },
};
