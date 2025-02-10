use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860510: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_510,
        source_type: SourceType::Wikidata,
        name: "Borland Reflex 2 color settings",
        extensions: &["r2z"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x65, 0x66, 0x6C, 0x65, 0x78, 0x20, 0x32, 0x00, 0x43, 0x6F, 0x6C,
                        0x6F, 0x72, 0x20, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x00,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
