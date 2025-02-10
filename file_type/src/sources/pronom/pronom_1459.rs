use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1459: FileType = FileType {
    file_format: &FileFormat {
        id: 1_459,
        source_type: SourceType::Pronom,
        name: "Adobe Type 1 Mac Font File",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(82),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4C, 0x57, 0x46, 0x4E]),
                        Token::WildcardCount(176),
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
