use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2868: FileType = FileType {
    file_format: &FileFormat {
        id: 2_868,
        source_type: SourceType::Pronom,
        name: "Sibelius Scorch",
        extensions: &["sco"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x43, 0x43, 0x53, 0x43, 0x4F, 0x52, 0x43, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
