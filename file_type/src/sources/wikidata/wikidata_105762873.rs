use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_873,
        source_type: SourceType::Wikidata,
        name: "V-BASE Virus data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x78, 0x48, 0x21, 0x0D, 0x0A, 0x46, 0x4E, 0x6E, 0x00, 0x56, 0x2D,
                        0x42, 0x61, 0x73, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
