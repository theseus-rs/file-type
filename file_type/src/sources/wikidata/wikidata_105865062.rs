use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865062: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_062,
        source_type: SourceType::Wikidata,
        name: "Professional Page document (v2.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x66, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x61, 0x6C,
                        0x20, 0x50, 0x61, 0x67, 0x65, 0x20, 0x56, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
