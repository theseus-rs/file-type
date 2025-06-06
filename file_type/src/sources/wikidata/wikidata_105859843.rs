use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859843: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_843,
        source_type: SourceType::Wikidata,
        name: "SlickEdit project",
        extensions: &["vpj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x50, 0x72,
                        0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D,
                        0x20, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77,
                        0x2E, 0x73, 0x6C, 0x69, 0x63, 0x6B, 0x65, 0x64, 0x69, 0x74, 0x2E, 0x63,
                        0x6F, 0x6D, 0x2F, 0x64, 0x74, 0x64, 0x2F, 0x76, 0x73, 0x65, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
