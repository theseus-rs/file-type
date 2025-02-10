use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1147: FileType = FileType {
    file_format: &FileFormat {
        id: 1_147,
        source_type: SourceType::Pronom,
        name: "Stuffit X Archive File",
        extensions: &["sitx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x21,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x02, 0x02, 0x55])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
