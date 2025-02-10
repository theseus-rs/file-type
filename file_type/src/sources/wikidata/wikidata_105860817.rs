use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860817: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_817,
        source_type: SourceType::Wikidata,
        name: "Reflex 2 Report",
        extensions: &["r2r"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x65, 0x66, 0x6C, 0x65, 0x78, 0x20, 0x32, 0x00, 0x52, 0x65, 0x70,
                        0x6F, 0x72, 0x74, 0x00, 0x66, 0x6F, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61,
                        0x62, 0x61, 0x73, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
