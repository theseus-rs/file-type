use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3932: FileType = FileType {
    file_format: &FileFormat {
        id: 3_932,
        source_type: SourceType::Pronom,
        name: "A2R Disk Image File",
        extensions: &["a2r"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x32, 0x52, 0x32, 0xFF, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
