use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2447: FileType = FileType {
    file_format: &FileFormat {
        id: 2_447,
        source_type: SourceType::Pronom,
        name: "Aero Studio Song",
        extensions: &["aero"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x41, 0x45, 0x52, 0x4F, 0x00, 0x00, 0x00, 0x01,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x40])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
