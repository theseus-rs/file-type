use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29000599: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_599,
        source_type: SourceType::Wikidata,
        name: "Windows Shortcut",
        extensions: &["lnk"],
        media_types: &["application/x-ms-shortcut"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
