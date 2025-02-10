use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1789: FileType = FileType {
    file_format: &FileFormat {
        id: 1_789,
        source_type: SourceType::Pronom,
        name: "Binary Property List",
        extensions: &["plist", "nib", "aae", "iMovieProj", "ezdraw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_670,
        }],
    },
};
