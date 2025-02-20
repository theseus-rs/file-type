use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858713: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_713,
        source_type: SourceType::Wikidata,
        name: "BrainLED AlfaWave session",
        extensions: &["baw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x72, 0x61, 0x69, 0x6E, 0x4C, 0x45, 0x44, 0x20, 0x41, 0x6C, 0x66,
                        0x61, 0x57, 0x61, 0x76, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
