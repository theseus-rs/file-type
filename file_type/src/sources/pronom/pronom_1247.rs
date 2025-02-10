use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1247: FileType = FileType {
    file_format: &FileFormat {
        id: 1_247,
        source_type: SourceType::Pronom,
        name: "Verity Collection Partition Definition Descriptor Style Set",
        extensions: &["pdd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x23, 0x20, 0x66, 0x61, 0x73, 0x68, 0x69, 0x6F, 0x6E, 0x2E, 0x70, 0x64,
                            0x64, 0x20, 0x2D, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E, 0x20,
                            0x62, 0x79, 0x20, 0x6D, 0x6B, 0x70, 0x72, 0x74, 0x69, 0x6E, 0x64, 0x20,
                            0x2D, 0x20, 0x53, 0x61, 0x74, 0x20, 0x44, 0x65, 0x63, 0x20, 0x30, 0x37,
                            0x20, 0x30, 0x30, 0x3A, 0x31, 0x32, 0x3A, 0x30, 0x32, 0x20, 0x31, 0x39,
                            0x39, 0x31, 0x0D, 0x0A,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7D, 0x0D, 0x0A, 0x7D, 0x0D, 0x0A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
