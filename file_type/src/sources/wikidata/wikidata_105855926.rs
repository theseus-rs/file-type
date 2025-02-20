use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855926: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_926,
        source_type: SourceType::Wikidata,
        name: "Klasik screen saver",
        extensions: &["drv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x2E, 0x54, 0x2E, 0x20, 0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20,
                        0x2D, 0x20, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x20, 0x53, 0x61, 0x76,
                        0x65, 0x72, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
