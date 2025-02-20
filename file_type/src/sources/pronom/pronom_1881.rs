use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1881: FileType = FileType {
    file_format: &FileFormat {
        id: 1_881,
        source_type: SourceType::Pronom,
        name: "AVCHD Playlist File",
        extensions: &["mpl", "mpls"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4D, 0x50, 0x4C, 0x53, 0x30, 0x31, 0x30, 0x30,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4D, 0x50, 0x4C, 0x53, 0x30, 0x32, 0x30, 0x30,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
