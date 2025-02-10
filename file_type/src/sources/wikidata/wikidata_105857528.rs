use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857528: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_528,
        source_type: SourceType::Wikidata,
        name: "CoLOS Image (V1)",
        extensions: &["image"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x21, 0x00, 0x2D, 0x00, 0x2D, 0x00, 0x57, 0x00,
                        0x72, 0x00, 0x69, 0x00, 0x74, 0x00, 0x74, 0x00, 0x65, 0x00, 0x6E, 0x00,
                        0x20, 0x00, 0x62, 0x00, 0x79, 0x00, 0x20, 0x00, 0x43, 0x00, 0x6F, 0x00,
                        0x4C, 0x00, 0x4F, 0x00, 0x53, 0x00, 0x20, 0x00, 0x43, 0x00, 0x72, 0x00,
                        0x65, 0x00, 0x61, 0x00, 0x74, 0x00, 0x65, 0x00, 0x20, 0x00, 0x56, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
