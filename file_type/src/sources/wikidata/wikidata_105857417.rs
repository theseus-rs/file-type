use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857417: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_417,
        source_type: SourceType::Wikidata,
        name: "JavaScript Flash Obfuscated",
        extensions: &["jsfl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x4D, 0x41, 0x43, 0x52, 0x4F, 0x4D, 0x45, 0x44, 0x49, 0x41,
                        0x4F, 0x42, 0x46, 0x55,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
