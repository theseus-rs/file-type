use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2689: FileType = FileType {
    file_format: &FileFormat {
        id: 2_689,
        source_type: SourceType::Pronom,
        name: "WordPerfect Presentations",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x0F, 0x0A, 0x02, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
