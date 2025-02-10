use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_264: FileType = FileType {
    file_format: &FileFormat {
        id: 264,
        source_type: SourceType::Pronom,
        name: "AMI Professional Document",
        extensions: &["sam"],
        media_types: &["application/vnd.lotus-wordpro"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x5B, 0x76, 0x65, 0x72, 0x5D]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x5B, 0x73, 0x74, 0x79, 0x5D]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x5B, 0x65, 0x64, 0x6F, 0x63, 0x5D]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
