use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860447: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_447,
        source_type: SourceType::Wikidata,
        name: "Remote Desktop Connection Settings (Unicode)",
        extensions: &["rdp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x73, 0x00, 0x63, 0x00, 0x72, 0x00, 0x65, 0x00, 0x65, 0x00,
                        0x6E, 0x00, 0x20, 0x00, 0x6D, 0x00, 0x6F, 0x00, 0x64, 0x00, 0x65, 0x00,
                        0x20, 0x00, 0x69, 0x00, 0x64, 0x00, 0x3A, 0x00, 0x69, 0x00, 0x3A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
