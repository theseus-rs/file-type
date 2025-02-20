use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_249,
        source_type: SourceType::Wikidata,
        name: "RedTitan Zip",
        extensions: &["rtz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x44, 0x54, 0x49, 0x54, 0x41, 0x4E, 0x20, 0x5A, 0x49, 0x50,
                        0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
