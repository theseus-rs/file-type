use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849997: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_997,
        source_type: SourceType::Wikidata,
        name: "GOLD Parser Tables",
        extensions: &["cgt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x00, 0x4F, 0x00, 0x4C, 0x00, 0x44, 0x00, 0x20, 0x00, 0x50, 0x00,
                        0x61, 0x00, 0x72, 0x00, 0x73, 0x00, 0x65, 0x00, 0x72, 0x00, 0x20, 0x00,
                        0x54, 0x00, 0x61, 0x00, 0x62, 0x00, 0x6C, 0x00, 0x65, 0x00, 0x73, 0x00,
                        0x2F, 0x00, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
