use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1779: FileType = FileType {
    file_format: &FileFormat {
        id: 1_779,
        source_type: SourceType::Pronom,
        name: "Notation Interchange File Format",
        extensions: &["nif"],
        media_types: &["application/vnd.music-niff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x4E, 0x49, 0x46, 0x46]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
