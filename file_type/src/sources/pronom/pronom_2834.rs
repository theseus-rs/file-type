use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2834: FileType = FileType {
    file_format: &FileFormat {
        id: 2_834,
        source_type: SourceType::Pronom,
        name: "Atrac Codec File",
        extensions: &["aea"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x08, 0x00, 0x00]),
                            Token::WildcardCount(260),
                            Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xAC])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
