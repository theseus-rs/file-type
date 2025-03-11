use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857045: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_045,
        source_type: SourceType::Wikidata,
        name: "GIOS Crunch compressed data (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x49, 0x4F, 0x53, 0x20, 0x52, 0x4C, 0x45, 0x20, 0x31, 0x2E, 0x30,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
