use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1373: FileType = FileType {
    file_format: &FileFormat {
        id: 1_373,
        source_type: SourceType::Pronom,
        name: "MPEG-2 Transport Stream",
        extensions: &["m2t", "ts", "m2ts"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                        Token::WildcardCount(187),
                        Token::Literal(&[0x47]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
