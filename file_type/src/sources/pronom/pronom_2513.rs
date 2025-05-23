use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2513: FileType = FileType {
    file_format: &FileFormat {
        id: 2_513,
        source_type: SourceType::Pronom,
        name: "Microsoft Office File List",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x78, 0x6D, 0x6C, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x6F,
                            0x3D, 0x22, 0x75, 0x72, 0x6E, 0x3A, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61,
                            0x73, 0x2D, 0x6D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x2D,
                            0x63, 0x6F, 0x6D, 0x3A, 0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x6F,
                            0x66, 0x66, 0x69, 0x63, 0x65, 0x22, 0x3E, 0x0D, 0x0A, 0x20, 0x3C, 0x6F,
                            0x3A, 0x4D, 0x61, 0x69, 0x6E, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x48, 0x52,
                            0x65, 0x66, 0x3D, 0x22, 0x2E, 0x2E,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x3C, 0x2F, 0x78, 0x6D, 0x6C, 0x3E])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
