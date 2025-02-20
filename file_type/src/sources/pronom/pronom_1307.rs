use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1307: FileType = FileType {
    file_format: &FileFormat {
        id: 1_307,
        source_type: SourceType::Pronom,
        name: "OpenType Font File",
        extensions: &["otf"],
        media_types: &["font/otf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x54, 0x54, 0x4F]),
                        Token::WildcardCountRange(8, 40),
                        Token::Literal(&[0x43, 0x46, 0x46, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
