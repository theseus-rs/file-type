use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1312: FileType = FileType {
    file_format: &FileFormat {
        id: 1_312,
        source_type: SourceType::Pronom,
        name: "Adobe Printer Font Binary",
        extensions: &["pfb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x80, 0x01]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x46, 0x6F,
                            0x6E, 0x74, 0x2D, 0x31, 0x2E, 0x30,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
