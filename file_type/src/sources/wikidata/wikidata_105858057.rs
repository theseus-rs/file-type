use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858057: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_057,
        source_type: SourceType::Wikidata,
        name: "IMPlay Song (v2.0)",
        extensions: &["iss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x4D, 0x50, 0x6C, 0x61, 0x79, 0x20, 0x53, 0x6F, 0x6E, 0x67, 0x20,
                        0x56, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
