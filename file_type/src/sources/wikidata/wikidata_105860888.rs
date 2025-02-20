use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_888,
        source_type: SourceType::Wikidata,
        name: "WinWay Resume (v4.0)",
        extensions: &["rsm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0E, 0x00, 0x01, 0x00, 0x25, 0x00, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
                        0x40, 0x40, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
                        0x09, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
