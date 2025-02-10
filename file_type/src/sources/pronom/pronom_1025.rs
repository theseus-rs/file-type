use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1025: FileType = FileType {
    file_format: &FileFormat {
        id: 1_025,
        source_type: SourceType::Pronom,
        name: "Gridded Binary",
        extensions: &["grb", "wmo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x47, 0x52, 0x49, 0x42]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x02]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x37, 0x37, 0x37, 0x37])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
