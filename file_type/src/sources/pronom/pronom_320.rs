use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_320: FileType = FileType {
    file_format: &FileFormat {
        id: 320,
        source_type: SourceType::Pronom,
        name: "Applixware Bitmap",
        extensions: &["im"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x2A]),
                            Token::Any(&[
                                &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E])],
                                &[Token::Literal(&[0x53, 0x54, 0x41, 0x52, 0x54])],
                            ]),
                            Token::Literal(&[
                                0x20, 0x52, 0x41, 0x53, 0x54, 0x45, 0x52, 0x20, 0x56, 0x45, 0x52,
                                0x53, 0x49, 0x4F, 0x4E, 0x3D,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x2A, 0x45, 0x4E, 0x44, 0x20, 0x52, 0x41, 0x53, 0x54, 0x45, 0x52, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
