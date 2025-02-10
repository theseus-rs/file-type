use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_534,
        source_type: SourceType::Wikidata,
        name: "RoboForm cached data",
        extensions: &["rfo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x66, 0x63, 0x61, 0x63, 0x68, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
