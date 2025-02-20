use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857031: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_031,
        source_type: SourceType::Wikidata,
        name: "Gliffy diagram (v1.x)",
        extensions: &["gliffy"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x63, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x54, 0x79, 0x70,
                        0x65, 0x22, 0x3A, 0x22, 0x61, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74,
                        0x69, 0x6F, 0x6E, 0x2F, 0x67, 0x6C, 0x69, 0x66, 0x66, 0x79, 0x2B, 0x6A,
                        0x73, 0x6F, 0x6E, 0x22, 0x2C, 0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x22, 0x3A, 0x22, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
