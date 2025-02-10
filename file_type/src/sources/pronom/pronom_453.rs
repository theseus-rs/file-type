use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_453: FileType = FileType {
    file_format: &FileFormat {
        id: 453,
        source_type: SourceType::Pronom,
        name: "X-Windows Screen Dump File",
        extensions: &["xdm", "xwd"],
        media_types: &["image/x-xwindowdump"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x06]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
