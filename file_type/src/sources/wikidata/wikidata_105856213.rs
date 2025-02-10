use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_213,
        source_type: SourceType::Wikidata,
        name: "Klasik Spreadsheet Import Driver",
        extensions: &["dsi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x2E, 0x54, 0x2E, 0x20, 0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20,
                        0x2D, 0x20, 0x69, 0x6D, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x64, 0x72, 0x69,
                        0x76, 0x65, 0x72, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
