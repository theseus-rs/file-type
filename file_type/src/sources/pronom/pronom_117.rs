use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_117: FileType = FileType {
    file_format: &FileFormat {
        id: 117,
        source_type: SourceType::Pronom,
        name: "Microsoft Outlook Personal Address Book",
        extensions: &["pab"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x21, 0x42, 0x44, 0x4E]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x41, 0x42]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
