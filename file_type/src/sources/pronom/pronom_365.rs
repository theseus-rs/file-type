use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_365: FileType = FileType {
    file_format: &FileFormat {
        id: 365,
        source_type: SourceType::Pronom,
        name: "Microsoft Outlook Personal Folders (Unicode)",
        extensions: &["pst"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x21, 0x42, 0x44, 0x4E]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x53, 0x4D, 0x17, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
