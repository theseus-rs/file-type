use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857806: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_806,
        source_type: SourceType::Wikidata,
        name: "Fahrenheit game data archive",
        extensions: &["dat", "idm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x55, 0x41, 0x4E, 0x54, 0x49, 0x43, 0x44, 0x52, 0x45, 0x41, 0x4D,
                        0x54, 0x41, 0x42, 0x49, 0x44, 0x4D, 0x45, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
