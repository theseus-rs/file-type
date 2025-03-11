use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762776: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_776,
        source_type: SourceType::Wikidata,
        name: "Chiasmus key",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x49, 0x53, 0x0A, 0x2A, 0x2A, 0x2A, 0x20, 0x41, 0x65, 0x6E, 0x64,
                        0x65, 0x72, 0x6E, 0x20, 0x53, 0x69, 0x65, 0x20, 0x64, 0x69, 0x65, 0x73,
                        0x65, 0x20, 0x44, 0x61, 0x74, 0x65, 0x69, 0x20, 0x6E, 0x69, 0x63, 0x68,
                        0x74, 0x20, 0x64, 0x69, 0x72, 0x65, 0x6B, 0x74, 0x21, 0x21, 0x20, 0x2A,
                        0x2A, 0x2A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
