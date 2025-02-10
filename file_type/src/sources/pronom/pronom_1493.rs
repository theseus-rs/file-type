use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1493: FileType = FileType {
    file_format: &FileFormat {
        id: 1_493,
        source_type: SourceType::Pronom,
        name: "Dalvik Executable Format",
        extensions: &["dex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x64, 0x65, 0x78, 0x0A]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
