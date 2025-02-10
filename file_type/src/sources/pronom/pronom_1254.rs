use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1254: FileType = FileType {
    file_format: &FileFormat {
        id: 1_254,
        source_type: SourceType::Pronom,
        name: "CorelDraw Drawing",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x57, 0x4C, 0x65, 0x00, 0x45, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(77),
                        Token::Literal(&[0x80, 0x40, 0x01, 0x00, 0x00, 0x80, 0x40]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
