use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1912: FileType = FileType {
    file_format: &FileFormat {
        id: 1_912,
        source_type: SourceType::Pronom,
        name: "Seattle FilmWorks SFW Image Format",
        extensions: &["sfw"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x39, 0x34, 0x41])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x39, 0x38, 0x41])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
