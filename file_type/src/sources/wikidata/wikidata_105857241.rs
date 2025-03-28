use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857241: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_241,
        source_type: SourceType::Wikidata,
        name: "Canadian Product Incident Report form",
        extensions: &["hcxs"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x66, 0x6F, 0x72, 0x6D, 0x22, 0x3A, 0x7B, 0x22, 0x66, 0x6F,
                        0x72, 0x6D, 0x49, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
                        0x22, 0x3A, 0x22, 0x43, 0x50, 0x53, 0x2D, 0x53, 0x50, 0x43, 0x2D, 0x30,
                        0x30, 0x30, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
