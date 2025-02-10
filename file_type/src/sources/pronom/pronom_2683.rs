use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2683: FileType = FileType {
    file_format: &FileFormat {
        id: 2_683,
        source_type: SourceType::Pronom,
        name: "3D Studio (DOS) Project File",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3D, 0xC2]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
