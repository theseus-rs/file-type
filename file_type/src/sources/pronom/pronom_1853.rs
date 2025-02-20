use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1853: FileType = FileType {
    file_format: &FileFormat {
        id: 1_853,
        source_type: SourceType::Pronom,
        name: "OGR GFS File",
        extensions: &["gfs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x47, 0x4D, 0x4C, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x43,
                            0x6C, 0x61, 0x73, 0x73, 0x4C, 0x69, 0x73, 0x74, 0x3E,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x2F, 0x47, 0x4D, 0x4C, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65,
                            0x43, 0x6C, 0x61, 0x73, 0x73, 0x4C, 0x69, 0x73, 0x74, 0x3E,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
