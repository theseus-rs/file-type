use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26085339: FileType = FileType {
    file_format: &FileFormat {
        id: 26_085_339,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format, version 1.0",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x30,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x25, 0x25, 0x45, 0x4F, 0x46])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
