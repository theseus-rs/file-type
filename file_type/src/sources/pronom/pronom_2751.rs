use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2751: FileType = FileType {
    file_format: &FileFormat {
        id: 2_751,
        source_type: SourceType::Pronom,
        name: "RagTime Document File",
        extensions: &["rtd", "rtt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x23, 0x2B, 0x44, 0xA4, 0x43, 0x4D, 0xA5, 0x48, 0x64, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
