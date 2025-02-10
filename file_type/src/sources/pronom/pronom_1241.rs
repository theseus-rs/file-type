use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1241: FileType = FileType {
    file_format: &FileFormat {
        id: 1_241,
        source_type: SourceType::Pronom,
        name: "Verity Collection Index About File",
        extensions: &["abt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x24, 0x63, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C, 0x3A, 0x20, 0x31,
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[
                            0x0A, 0x63, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x5F,
                            0x61, 0x62, 0x6F, 0x75, 0x74, 0x3A,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
