use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2174: FileType = FileType {
    file_format: &FileFormat {
        id: 2_174,
        source_type: SourceType::Pronom,
        name: "Virtual Format (Raster)",
        extensions: &["vrt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x56, 0x52, 0x54, 0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x20,
                        0x72, 0x61, 0x73, 0x74, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
