use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851107: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_107,
        source_type: SourceType::Wikidata,
        name: "TePee Animator Project (v1.0)",
        extensions: &["tap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x54, 0x65, 0x50, 0x65, 0x65, 0x20, 0x41, 0x6E, 0x69, 0x6D, 0x61,
                        0x74, 0x6F, 0x72, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x2E,
                        0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
