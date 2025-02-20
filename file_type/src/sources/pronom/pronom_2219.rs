use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2219: FileType = FileType {
    file_format: &FileFormat {
        id: 2_219,
        source_type: SourceType::Pronom,
        name: "Student Writing Center Report",
        extensions: &["rp", "rpt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x1A, 0x54, 0x4C, 0x43]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x46, 0x46, 0x00, 0x00, 0x00]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x1A, 0x46, 0x46, 0x1A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
