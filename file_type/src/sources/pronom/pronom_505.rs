use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_505: FileType = FileType {
    file_format: &FileFormat {
        id: 505,
        source_type: SourceType::Pronom,
        name: "Macromedia Director",
        extensions: &["dir", "dxr"],
        media_types: &["application/x-director"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x4D, 0x56, 0x39, 0x33, 0x69, 0x6D, 0x61, 0x70, 0x00, 0x00, 0x00, 0x18,
                            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x2C,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
