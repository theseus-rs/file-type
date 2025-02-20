use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_621,
        source_type: SourceType::Wikidata,
        name: "Roxio CD Image format (v3)",
        extensions: &["c2d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x6F, 0x78, 0x69, 0x6F, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20,
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20,
                        0x33, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
