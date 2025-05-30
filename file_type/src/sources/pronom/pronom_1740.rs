use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1740: FileType = FileType {
    file_format: &FileFormat {
        id: 1_740,
        source_type: SourceType::Pronom,
        name: "Animated Portable Network Graphics",
        extensions: &["png", "apng"],
        media_types: &["image/vnd.mozilla.apng"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00,
                                0x0D, 0x49, 0x48, 0x44, 0x52,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x61, 0x63, 0x54, 0x4C]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 664,
        }],
    },
};
