use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27966959: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_959,
        source_type: SourceType::Wikidata,
        name: "YM",
        extensions: &["ym", "ymst"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x59, 0x4D, 0x53, 0x54, 0x80, 0x00, 0x59])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x59, 0x4D, 0x36, 0x21, 0x4C, 0x65, 0x4F, 0x6E, 0x41, 0x72, 0x44, 0x21,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
