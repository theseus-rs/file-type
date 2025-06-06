use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855765: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_765,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Picture/drawing (v2)",
        extensions: &["dpp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x7B, 0x44, 0x72, 0x61, 0x77, 0x50, 0x6C, 0x75, 0x73, 0x20, 0x50,
                        0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x56, 0x32, 0x20, 0x44, 0x50,
                        0x20, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
