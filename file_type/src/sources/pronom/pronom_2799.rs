use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2799: FileType = FileType {
    file_format: &FileFormat {
        id: 2_799,
        source_type: SourceType::Pronom,
        name: "Lotus Screencam Data File",
        extensions: &["scm"],
        media_types: &["application/vnd.lotus-screencam"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x80, 0x53, 0x43, 0x4D, 0x4F, 0x56, 0x76, 0x33, 0x2E]),
                        Token::Any(&[&[Token::Literal(&[0x30])], &[Token::Literal(&[0x31])]]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x30, 0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
