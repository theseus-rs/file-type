use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2757: FileType = FileType {
    file_format: &FileFormat {
        id: 2_757,
        source_type: SourceType::Pronom,
        name: "SGI Movie File",
        extensions: &["mv", "movie"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x56, 0x49, 0x00, 0x00, 0x00, 0x03,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
