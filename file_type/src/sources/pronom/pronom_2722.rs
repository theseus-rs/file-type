use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2722: FileType = FileType {
    file_format: &FileFormat {
        id: 2_722,
        source_type: SourceType::Pronom,
        name: "Leapfrog Geo 3D Scene Format",
        extensions: &["lfsc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x4C, 0x65, 0x61, 0x70, 0x66, 0x72, 0x6F, 0x67, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
