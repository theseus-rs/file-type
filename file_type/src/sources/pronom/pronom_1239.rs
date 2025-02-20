use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1239: FileType = FileType {
    file_format: &FileFormat {
        id: 1_239,
        source_type: SourceType::Pronom,
        name: "Acrobat Catalog Cat File",
        extensions: &["cat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x41, 0x0D, 0x0A])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x41, 0x0D, 0x0A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
