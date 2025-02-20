use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850722: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_722,
        source_type: SourceType::Wikidata,
        name: "Kaleidoscope Kreator 3 Template",
        extensions: &["ktp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x02, 0x00, 0x00, 0x00, 0x50, 0x4B,
                        0x61, 0x6C, 0x4B, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x2E, 0x4B, 0x61,
                        0x6C, 0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x73, 0x2C, 0x20,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x33, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
