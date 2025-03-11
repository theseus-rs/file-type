use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851492: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_492,
        source_type: SourceType::Wikidata,
        name: "T-Tracks preset (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x03, 0xE8, 0x00, 0x0B, 0x06, 0x80, 0x00, 0x00, 0x03, 0xE8,
                        0x54, 0x50, 0x52, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
