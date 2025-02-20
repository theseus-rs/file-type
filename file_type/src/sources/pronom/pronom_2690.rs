use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2690: FileType = FileType {
    file_format: &FileFormat {
        id: 2_690,
        source_type: SourceType::Pronom,
        name: "Leica Project File",
        extensions: &["lgs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x77, 0x69, 0x6C, 0x64, 0x3A, 0x3A, 0x67, 0x65, 0x6F, 0x73, 0x79, 0x73,
                            0x74, 0x65, 0x6D, 0x73,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4C, 0x65, 0x69, 0x63, 0x61, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                            0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
