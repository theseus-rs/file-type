use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_388: FileType = FileType {
    file_format: &FileFormat {
        id: 388,
        source_type: SourceType::Pronom,
        name: "BZIP2 Compressed Archive",
        extensions: &["bz2"],
        media_types: &["application/x-bzip2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x5A, 0x68]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x31, 0x41, 0x59, 0x26, 0x53, 0x59]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_878,
        }],
    },
};
