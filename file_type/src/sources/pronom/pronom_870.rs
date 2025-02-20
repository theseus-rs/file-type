use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_870: FileType = FileType {
    file_format: &FileFormat {
        id: 870,
        source_type: SourceType::Pronom,
        name: "Microsoft Internet Shortcut",
        extensions: &["url"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x6E, 0x65, 0x74, 0x53, 0x68, 0x6F,
                        0x72, 0x74, 0x63, 0x75, 0x74, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
