use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2233: FileType = FileType {
    file_format: &FileFormat {
        id: 2_233,
        source_type: SourceType::Pronom,
        name: "GST Publisher File",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x44, 0x54, 0x50, 0x49])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x45, 0x4F, 0x44, 0x46])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
