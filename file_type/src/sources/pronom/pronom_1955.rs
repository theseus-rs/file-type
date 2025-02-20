use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1955: FileType = FileType {
    file_format: &FileFormat {
        id: 1_955,
        source_type: SourceType::Pronom,
        name: "Maxwell Render Material File",
        extensions: &["mxm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x58, 0x4D]),
                        Token::WildcardCountRange(4, 64),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
