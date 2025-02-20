use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_478705: FileType = FileType {
    file_format: &FileFormat {
        id: 478_705,
        source_type: SourceType::Wikidata,
        name: "proxy auto-config",
        extensions: &["pac"],
        media_types: &[
            "application/x-javascript-config",
            "application/x-ns-proxy-autoconfig",
        ],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x66, 0x75, 0x6E, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6E,
                            0x64, 0x50, 0x72, 0x6F, 0x78, 0x79, 0x46, 0x6F, 0x72, 0x55, 0x52, 0x4C,
                            0x28, 0x75, 0x72, 0x6C, 0x2C, 0x20, 0x68, 0x6F, 0x73, 0x74, 0x29,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x2F, 0x2A])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
