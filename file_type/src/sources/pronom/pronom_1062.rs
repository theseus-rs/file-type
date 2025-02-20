use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1062: FileType = FileType {
    file_format: &FileFormat {
        id: 1_062,
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
                        Token::Literal(&[0x58, 0x46, 0x49, 0x52]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x33, 0x39, 0x56, 0x4D, 0x70, 0x61, 0x6D, 0x69, 0x18, 0x00, 0x00, 0x00,
                            0x01, 0x00, 0x00, 0x00,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
