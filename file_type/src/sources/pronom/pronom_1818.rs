use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1818: FileType = FileType {
    file_format: &FileFormat {
        id: 1_818,
        source_type: SourceType::Pronom,
        name: "INTERLIS Transfer File",
        extensions: &["itf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x53, 0x43, 0x4E, 0x54])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x45, 0x4E, 0x44, 0x45])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
