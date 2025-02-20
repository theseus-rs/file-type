use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1958: FileType = FileType {
    file_format: &FileFormat {
        id: 1_958,
        source_type: SourceType::Pronom,
        name: "Maxwell Render Scene File Format",
        extensions: &["mxs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xF9, 0xB2, 0x2A, 0xD1]),
                        Token::WildcardCountRange(4, 32),
                        Token::Literal(&[0x6D, 0x78, 0x73, 0x74]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
