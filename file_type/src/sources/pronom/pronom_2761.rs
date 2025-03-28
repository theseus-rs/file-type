use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2761: FileType = FileType {
    file_format: &FileFormat {
        id: 2_761,
        source_type: SourceType::Pronom,
        name: "Universal Scene Description ASCII File",
        extensions: &["usda"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x75, 0x73, 0x64, 0x61, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
