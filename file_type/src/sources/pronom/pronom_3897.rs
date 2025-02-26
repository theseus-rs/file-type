use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3897: FileType = FileType {
    file_format: &FileFormat {
        id: 3_897,
        source_type: SourceType::Pronom,
        name: "Parquet File",
        extensions: &["parquet"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x41, 0x52, 0x31])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x41, 0x52, 0x31])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
