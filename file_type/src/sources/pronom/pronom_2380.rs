use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2380: FileType = FileType {
    file_format: &FileFormat {
        id: 2_380,
        source_type: SourceType::Pronom,
        name: "Standard Data Format",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x47, 0x30])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x0D, 0x0A, 0x1A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
