use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_534,
        source_type: SourceType::Wikidata,
        name: "BAMZOOKi creature data",
        extensions: &["zook"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6F, 0x6E, 0x73, 0x61, 0x69, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E,
                        0x65, 0x20, 0x2D, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x64,
                        0x20, 0x69, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3A, 0x20, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                        0x31, 0x2E, 0x20, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
