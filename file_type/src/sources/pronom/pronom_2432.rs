use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2432: FileType = FileType {
    file_format: &FileFormat {
        id: 2_432,
        source_type: SourceType::Pronom,
        name: "True Colour Sprites [Spooky Sprites] (Atari Falcon)",
        extensions: &["trs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x43, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 2_433,
        }],
    },
};
