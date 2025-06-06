use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2808: FileType = FileType {
    file_format: &FileFormat {
        id: 2_808,
        source_type: SourceType::Pronom,
        name: "Draw.io Diagram (XML) File",
        extensions: &["drawio", "xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x3C, 0x6D, 0x78, 0x66, 0x69, 0x6C, 0x65])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x2F, 0x64, 0x69, 0x61, 0x67, 0x72, 0x61, 0x6D, 0x3E, 0x3C, 0x2F,
                            0x6D, 0x78, 0x66, 0x69, 0x6C, 0x65, 0x3E,
                        ])],
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
