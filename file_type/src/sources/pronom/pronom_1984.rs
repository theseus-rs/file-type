use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1984: FileType = FileType {
    file_format: &FileFormat {
        id: 1_984,
        source_type: SourceType::Pronom,
        name: "Hewlett Packard Graphics Language",
        extensions: &["000"],
        media_types: &["application/vnd.hp-HPGL"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x1B, 0x25, 0x30, 0x42, 0x42, 0x50, 0x49, 0x4E,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x47, 0x3B])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 446,
        }],
    },
};
