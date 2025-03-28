use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851783: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_783,
        source_type: SourceType::Wikidata,
        name: "K-Spreadsheet (v2)",
        extensions: &["spd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6C, 0x69, 0x66, 0x66, 0x20, 0x48, 0x41, 0x52, 0x4B, 0x45, 0x52,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
