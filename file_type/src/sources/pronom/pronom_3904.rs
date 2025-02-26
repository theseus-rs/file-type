use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3904: FileType = FileType {
    file_format: &FileFormat {
        id: 3_904,
        source_type: SourceType::Pronom,
        name: "Apache ORC",
        extensions: &["orc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4F, 0x52, 0x43])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4F, 0x52, 0x43]),
                            Token::Any(&[
                                &[Token::Literal(&[0x14])],
                                &[Token::Literal(&[0x15])],
                                &[Token::Literal(&[0x16])],
                                &[Token::Literal(&[0x17])],
                                &[Token::Literal(&[0x18])],
                            ]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
