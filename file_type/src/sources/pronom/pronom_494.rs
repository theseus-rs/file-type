use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_494: FileType = FileType {
    file_format: &FileFormat {
        id: 494,
        source_type: SourceType::Pronom,
        name: "Lotus 1-2-3 Spreadsheet Formatting File",
        extensions: &["fm1", "fmt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x06, 0x80]),
                        Token::Any(&[
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x83])],
                            &[Token::Literal(&[0x96])],
                        ]),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
