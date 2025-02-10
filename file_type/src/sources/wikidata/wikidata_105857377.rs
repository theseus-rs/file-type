use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857377: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_377,
        source_type: SourceType::Wikidata,
        name: "Trizbort.io map",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x22, 0x3A,
                        0x7B, 0x22, 0x67, 0x72, 0x69, 0x64, 0x22, 0x3A, 0x7B, 0x22, 0x76, 0x69,
                        0x73, 0x69, 0x62, 0x6C, 0x65, 0x22, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
