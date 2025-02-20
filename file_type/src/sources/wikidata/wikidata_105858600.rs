use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858600: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_600,
        source_type: SourceType::Wikidata,
        name: "BrainWave Generator",
        extensions: &["bwg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x72, 0x61, 0x69, 0x6E, 0x57, 0x61, 0x76, 0x65, 0x20, 0x47, 0x65,
                        0x6E, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x2C, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69,
                        0x74, 0x68, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
