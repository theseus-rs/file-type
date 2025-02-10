use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_79238203: FileType = FileType {
    file_format: &FileFormat {
        id: 79_238_203,
        source_type: SourceType::Wikidata,
        name: "Adobe Authorware Library",
        extensions: &["a4l"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4C, 0x49, 0x42, 0xBE, 0xB3, 0xB6, 0xBD, 0x16, 0x00, 0x00, 0x00,
                        0xF9, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
