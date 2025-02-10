use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1329: FileType = FileType {
    file_format: &FileFormat {
        id: 1_329,
        source_type: SourceType::Pronom,
        name: "GEM Metafile Format",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xFF, 0x00]),
                            Token::Any(&[&[Token::Literal(&[0x18])], &[Token::Literal(&[0x0E])]]),
                            Token::Literal(&[0x00, 0x65, 0x00]),
                            Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xFF]),
                            Token::Any(&[&[Token::Literal(&[0x18])], &[Token::Literal(&[0x0E])]]),
                            Token::Literal(&[0x00, 0x65, 0x00]),
                            Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
                            Token::Literal(&[0x00]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_330,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 304,
            },
        ],
    },
};
