use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1972: FileType = FileType {
    file_format: &FileFormat {
        id: 1_972,
        source_type: SourceType::Pronom,
        name: "Folio Flat File",
        extensions: &["fff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x3C, 0x43, 0x4D, 0x3E])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x46, 0x6F, 0x6C, 0x69, 0x6F, 0x2C, 0x46, 0x46, 0x46,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
