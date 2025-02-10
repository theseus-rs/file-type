use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1846: FileType = FileType {
    file_format: &FileFormat {
        id: 1_846,
        source_type: SourceType::Pronom,
        name: "HDF",
        extensions: &["hdf", "h4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0E, 0x03, 0x13, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
