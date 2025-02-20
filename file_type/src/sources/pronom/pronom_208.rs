use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_208: FileType = FileType {
    file_format: &FileFormat {
        id: 208,
        source_type: SourceType::Pronom,
        name: "Paradox Database Table",
        extensions: &["db"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x08]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                        Token::Range(&[0x01], &[0x20]),
                        Token::WildcardCount(51),
                        Token::Literal(&[0x0C]),
                        Token::WildcardCount(34),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
