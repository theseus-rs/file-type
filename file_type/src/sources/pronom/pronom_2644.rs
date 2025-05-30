use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2644: FileType = FileType {
    file_format: &FileFormat {
        id: 2_644,
        source_type: SourceType::Pronom,
        name: "JPEG 2000 Codestream",
        extensions: &["j2k", "jpc", "j2c"],
        media_types: &["image/jp2"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0x4F, 0xFF, 0x51])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xD9])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
