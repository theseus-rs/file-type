use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860667: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_667,
        source_type: SourceType::Wikidata,
        name: "Remote Desktop Connection Settings",
        extensions: &["rdp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x20,
                        0x69, 0x64, 0x3A, 0x69, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
