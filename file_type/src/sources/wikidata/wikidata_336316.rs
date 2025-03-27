use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_336316: FileType = FileType {
    file_format: &FileFormat {
        id: 336_316,
        source_type: SourceType::Wikidata,
        name: "MPEG-4 Part 14",
        extensions: &["mp4"],
        media_types: &["audio/mp4", "video/mp4"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x66, 0x74, 0x79, 0x70, 0x4D, 0x34, 0x41, 0x20,
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
                            0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x4D, 0x34, 0x41, 0x20,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
