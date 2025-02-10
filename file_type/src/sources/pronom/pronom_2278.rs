use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2278: FileType = FileType {
    file_format: &FileFormat {
        id: 2_278,
        source_type: SourceType::Pronom,
        name: "Primavera P6 Project Management XER File",
        extensions: &["xer"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x45, 0x52, 0x4D, 0x48, 0x44, 0x52])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x0D, 0x0A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
