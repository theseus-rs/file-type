use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_784,
        source_type: SourceType::Wikidata,
        name: "ProForm Database",
        extensions: &["pfd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x66, 0x74, 0x50, 0x72, 0x6F, 0x27, 0x73, 0x20, 0x50, 0x72,
                        0x6F, 0x46, 0x6F, 0x72, 0x6D, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x57, 0x69,
                        0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x28, 0x2E, 0x50, 0x46, 0x44, 0x29,
                        0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
