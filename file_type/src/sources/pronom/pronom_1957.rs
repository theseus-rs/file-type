use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1957: FileType = FileType {
    file_format: &FileFormat {
        id: 1_957,
        source_type: SourceType::Pronom,
        name: "SIDOUN WinAVA Format",
        extensions: &["swa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x41, 0x56, 0x41, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
