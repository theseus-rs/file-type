use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1108: FileType = FileType {
    file_format: &FileFormat {
        id: 1_108,
        source_type: SourceType::Pronom,
        name: "pulse EKKO header file",
        extensions: &["hd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x4E, 0x55, 0x4D, 0x42, 0x45, 0x52, 0x20, 0x4F, 0x46, 0x20, 0x54,
                                0x52, 0x41, 0x43, 0x45, 0x53, 0x20, 0x20, 0x20, 0x3D, 0x20,
                            ]),
                            Token::Range(&[0x30], &[0x39]),
                            Token::WildcardCountRange(3, 4_294_967_295),
                            Token::Literal(&[
                                0x4E, 0x55, 0x4D, 0x42, 0x45, 0x52, 0x20, 0x4F, 0x46, 0x20, 0x50,
                                0x54, 0x53, 0x2F, 0x54, 0x52, 0x43, 0x20, 0x20, 0x3D, 0x20,
                            ]),
                            Token::Range(&[0x30], &[0x39]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x53, 0x55, 0x52, 0x56, 0x45, 0x59, 0x20, 0x4D, 0x4F, 0x44, 0x45, 0x20,
                            0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3D, 0x20,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
