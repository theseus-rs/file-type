use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853482: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_482,
        source_type: SourceType::Wikidata,
        name: "ZX1 bitstream/core",
        extensions: &["zx1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                        0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0x99, 0x55, 0x66, 0x30, 0xA1, 0x00, 0x07,
                        0x20, 0x00, 0x31, 0xA1, 0x03, 0x80, 0x31, 0x41, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
