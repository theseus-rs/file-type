use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854865: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_865,
        source_type: SourceType::Wikidata,
        name: "ArduPilot JSON firmware",
        extensions: &["apj", "px4", "vrx"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x22, 0x62, 0x6F, 0x61, 0x72, 0x64,
                        0x5F, 0x69, 0x64, 0x22, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
