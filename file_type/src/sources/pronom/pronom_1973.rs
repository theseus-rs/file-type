use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1973: FileType = FileType {
    file_format: &FileFormat {
        id: 1_973,
        source_type: SourceType::Pronom,
        name: "Folio Definition File",
        extensions: &["def"],
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
                            0x46, 0x6F, 0x6C, 0x69, 0x6F, 0x2C, 0x44, 0x45, 0x46,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
