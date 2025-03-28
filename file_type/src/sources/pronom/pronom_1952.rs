use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1952: FileType = FileType {
    file_format: &FileFormat {
        id: 1_952,
        source_type: SourceType::Pronom,
        name: "VectorWorks Plugin or Script",
        extensions: &["vso", "vst", "vsm"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x56, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
