use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856632: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_632,
        source_type: SourceType::Wikidata,
        name: "wxMaxima batch (v1)",
        extensions: &["wxm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x20, 0x5B, 0x77, 0x78, 0x4D, 0x61, 0x78, 0x69, 0x6D, 0x61,
                        0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x5D, 0x20, 0x5B,
                        0x20, 0x44, 0x4F, 0x20, 0x4E, 0x4F, 0x54, 0x20, 0x45, 0x44, 0x49, 0x54,
                        0x20, 0x42, 0x59, 0x20, 0x48, 0x41, 0x4E, 0x44, 0x21, 0x20, 0x5D, 0x2A,
                        0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
