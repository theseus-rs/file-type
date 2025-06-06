use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1363: FileType = FileType {
    file_format: &FileFormat {
        id: 1_363,
        source_type: SourceType::Pronom,
        name: "GraphPad Prism",
        extensions: &["pzm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x46, 0x46, 0x47, 0x52, 0x41, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
