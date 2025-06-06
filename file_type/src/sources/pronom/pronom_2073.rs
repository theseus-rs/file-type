use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2073: FileType = FileType {
    file_format: &FileFormat {
        id: 2_073,
        source_type: SourceType::Pronom,
        name: "Windows Address Book",
        extensions: &["wab"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x9C, 0xCB, 0xCB, 0x8D, 0x13, 0x75, 0xD2, 0x11, 0x91, 0x58, 0x00, 0xC0,
                            0x4F, 0x79, 0x56, 0xA4,
                        ]),
                        Token::WildcardCountRange(0, 400),
                        Token::Literal(&[
                            0x81, 0x32, 0x84, 0xC1, 0x85, 0x05, 0xD0, 0x11, 0xB2, 0x90, 0x00, 0xAA,
                            0x00, 0x3C, 0xF6, 0x76,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
