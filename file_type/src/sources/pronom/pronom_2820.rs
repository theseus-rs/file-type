use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2820: FileType = FileType {
    file_format: &FileFormat {
        id: 2_820,
        source_type: SourceType::Pronom,
        name: "Graphisoft Archicad Project",
        extensions: &["pla", "pln"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x6D, 0x6D]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
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
                            Token::Literal(&[0x4D, 0x4D]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
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
                            Token::Literal(&[0x57, 0x57]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
