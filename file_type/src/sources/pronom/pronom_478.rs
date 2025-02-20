use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_478: FileType = FileType {
    file_format: &FileFormat {
        id: 478,
        source_type: SourceType::Pronom,
        name: "FileMaker Pro Database",
        extensions: &["fp5", "fmp", "fp", "fm"],
        media_types: &[],
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
                        Token::Literal(&[0x50, 0x72, 0x6F, 0x20, 0x35]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
