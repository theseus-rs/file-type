use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2242: FileType = FileType {
    file_format: &FileFormat {
        id: 2_242,
        source_type: SourceType::Pronom,
        name: "WordPerfect Encrypted Document",
        extensions: &["wp"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xFF, 0x61, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
