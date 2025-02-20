use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_389: FileType = FileType {
    file_format: &FileFormat {
        id: 389,
        source_type: SourceType::Pronom,
        name: "ZOO Compressed Archive",
        extensions: &["zoo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(20),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xDC, 0xA7, 0xC4, 0xFD])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFC, 0x83])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
