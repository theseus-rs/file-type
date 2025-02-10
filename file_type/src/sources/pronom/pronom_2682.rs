use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2682: FileType = FileType {
    file_format: &FileFormat {
        id: 2_682,
        source_type: SourceType::Pronom,
        name: "3D Studio (DOS) 2D/3D Loft Object File",
        extensions: &["lft"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3D, 0x2D]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
