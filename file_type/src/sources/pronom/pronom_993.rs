use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_993: FileType = FileType {
    file_format: &FileFormat {
        id: 993,
        source_type: SourceType::Pronom,
        name: "DjVu File Format",
        extensions: &["djvu", "djv"],
        media_types: &["image/vnd.djvu", "image/x-djvu"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x44, 0x4A, 0x56, 0x4D, 0x44, 0x49, 0x52, 0x4D,
                            ])],
                            &[Token::Literal(&[
                                0x44, 0x4A, 0x56, 0x55, 0x49, 0x4E, 0x46, 0x4F,
                            ])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
