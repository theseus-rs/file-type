use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857036: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_036,
        source_type: SourceType::Wikidata,
        name: "GIOS Compress compressed data (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x49, 0x4F, 0x53, 0x20, 0x43, 0x4D, 0x50, 0x20, 0x31, 0x2E, 0x30,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
