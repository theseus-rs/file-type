use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856635: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_635,
        source_type: SourceType::Wikidata,
        name: "KingSoft WPS98 document",
        extensions: &["wps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x82, 0xFF, 0x57, 0x50, 0x53, 0x39, 0x38, 0xAE, 0x20, 0x66, 0x6F, 0x72,
                        0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x4B, 0x69, 0x6E, 0x67,
                        0x73, 0x6F, 0x66, 0x74, 0xA9, 0x20, 0x31, 0x39, 0x39, 0x38, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
