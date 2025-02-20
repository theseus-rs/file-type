use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1079: FileType = FileType {
    file_format: &FileFormat {
        id: 1_079,
        source_type: SourceType::Pronom,
        name: "Crystallographic Information Framework",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x5F, 0x61, 0x74, 0x6F, 0x6D, 0x5F, 0x74, 0x79, 0x70, 0x65, 0x5F,
                        0x73, 0x63, 0x61, 0x74, 0x5F, 0x64, 0x69, 0x73, 0x70, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x5F, 0x72, 0x65, 0x61, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
