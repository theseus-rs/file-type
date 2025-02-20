use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1836: FileType = FileType {
    file_format: &FileFormat {
        id: 1_836,
        source_type: SourceType::Pronom,
        name: "Stata Data (DTA) Format",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x6E]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                        Token::Literal(&[0x01]),
                        Token::WildcardCount(105),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
