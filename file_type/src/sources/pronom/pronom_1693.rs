use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1693: FileType = FileType {
    file_format: &FileFormat {
        id: 1_693,
        source_type: SourceType::Pronom,
        name: "Feather",
        extensions: &["feather"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x45, 0x41, 0x31])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x45, 0x41, 0x31])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
