use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850590: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_590,
        source_type: SourceType::Wikidata,
        name: "MacStitch/WinStitch design",
        extensions: &["chart"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x41, 0x6E, 0x20, 0x55, 0x72, 0x73, 0x61, 0x20, 0x53, 0x6F,
                        0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2C,
                        0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
