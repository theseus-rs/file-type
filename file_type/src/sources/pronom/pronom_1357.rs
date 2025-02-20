use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1357: FileType = FileType {
    file_format: &FileFormat {
        id: 1_357,
        source_type: SourceType::Pronom,
        name: "Matroska",
        extensions: &["mkv", "mk3d", "mka", "mks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3]),
                        Token::WildcardCountRange(0, 32),
                        Token::Literal(&[
                            0x42, 0x82, 0x88, 0x6D, 0x61, 0x74, 0x72, 0x6F, 0x73, 0x6B, 0x61, 0x42,
                            0x87,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        }],
    },
};
