use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2419: FileType = FileType {
    file_format: &FileFormat {
        id: 2_419,
        source_type: SourceType::Pronom,
        name: "ASEG-GDF2 Description File",
        extensions: &["des"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x52, 0x4F, 0x4A, 0x47, 0x44, 0x41])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x0D, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
