use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2329: FileType = FileType {
    file_format: &FileFormat {
        id: 2_329,
        source_type: SourceType::Pronom,
        name: "EinScan RGE 3D Range File",
        extensions: &["rge"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x44, 0x53, 0x63, 0x61, 0x6E, 0x33, 0x2E, 0x30, 0x20, 0x52, 0x61,
                        0x6E, 0x67, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
