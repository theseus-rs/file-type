use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_80: FileType = FileType {
    file_format: &FileFormat {
        id: 80,
        source_type: SourceType::Pronom,
        name: "AutoCAD Design Web Format",
        extensions: &["dwf"],
        media_types: &[
            "application/dwf",
            "application/x-dwf",
            "drawing/x-dwf",
            "image/vnd.dwf",
            "image/x-dwf",
            "model/vnd.dwf",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x28, 0x44, 0x57, 0x46, 0x20, 0x56, 0x30]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x2E]),
                        Token::SingleWildcard,
                        Token::SingleWildcard,
                        Token::Literal(&[0x29]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
