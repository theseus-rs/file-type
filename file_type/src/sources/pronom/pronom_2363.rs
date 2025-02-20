use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2363: FileType = FileType {
    file_format: &FileFormat {
        id: 2_363,
        source_type: SourceType::Pronom,
        name: "Raster Matrix Format",
        extensions: &["rsw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x57, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
