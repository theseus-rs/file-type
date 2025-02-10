use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2321: FileType = FileType {
    file_format: &FileFormat {
        id: 2_321,
        source_type: SourceType::Pronom,
        name: "Cool Edit/Adobe Audition Session File",
        extensions: &["ses"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4F, 0x4C, 0x4E, 0x45, 0x53, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
