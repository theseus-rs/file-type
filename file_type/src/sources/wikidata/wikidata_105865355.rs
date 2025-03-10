use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_355,
        source_type: SourceType::Wikidata,
        name: "PC-Calc spreadsheet (v3.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x2D, 0x43, 0x41, 0x4C, 0x43, 0x20, 0x33, 0x2E, 0x30, 0x0D,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
