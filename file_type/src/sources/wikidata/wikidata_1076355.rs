use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1076355: FileType = FileType {
    file_format: &FileFormat {
        id: 1_076_355,
        source_type: SourceType::Wikidata,
        name: "Portable Executable",
        extensions: &[
            "acm", "ax", "dll", "drv", "efi", "exe", "mui", "scr", "sys", "tsp",
        ],
        media_types: &[
            "application/efi",
            "application/vnd.microsoft.portable-executable",
        ],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D, 0x5A]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4D, 0x5A])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
