use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_806: FileType = FileType {
    file_format: &FileFormat {
        id: 806,
        source_type: SourceType::Pronom,
        name: "DVD data file and backup data file",
        extensions: &["ifo", "bup"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x44, 0x56, 0x44, 0x56, 0x49, 0x44, 0x45, 0x4F, 0x2D, 0x56,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x4D, 0x47])],
                            &[Token::Literal(&[0x54, 0x53])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
