use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863876: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_876,
        source_type: SourceType::Wikidata,
        name: "Mouse Calc spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8C, 0x95, 0x83, 0x00, 0x38, 0xE9, 0x01, 0xD0, 0xFB, 0x88, 0xD0, 0xF6,
                        0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xAA, 0xD0, 0xF7, 0xEA, 0xBD, 0x8C,
                        0xC0, 0x30, 0x04, 0xBD, 0x88, 0xC0, 0x60, 0xA9, 0x00, 0x8D, 0xF4, 0x03,
                        0x20, 0x80, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
