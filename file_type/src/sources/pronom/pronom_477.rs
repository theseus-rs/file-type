use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_477: FileType = FileType {
    file_format: &FileFormat {
        id: 477,
        source_type: SourceType::Pronom,
        name: "FileMaker Pro Database",
        extensions: &["fp3", "fmp", "fp", "fm"],
        media_types: &["application/x-filemaker"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x02,
                            0x00, 0x02, 0xC0,
                        ]),
                        Token::WildcardCount(527),
                        Token::Literal(&[0x50, 0x72, 0x6F, 0x20, 0x33]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
