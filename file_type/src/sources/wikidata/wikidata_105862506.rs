use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_506,
        source_type: SourceType::Wikidata,
        name: "MonoDevelop Solution",
        extensions: &["mdsx"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4D, 0x6F, 0x6E, 0x6F, 0x44, 0x65, 0x76, 0x65, 0x6C, 0x6F, 0x70,
                        0x53, 0x6F, 0x6C, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E,
                        0x30, 0x22, 0x3E, 0x0A, 0x20, 0x20, 0x3C, 0x52, 0x65, 0x6C, 0x61, 0x74,
                        0x69, 0x76, 0x65, 0x4F, 0x75, 0x74, 0x70, 0x75, 0x74, 0x50, 0x61, 0x74,
                        0x68, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
