use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851999: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_999,
        source_type: SourceType::Wikidata,
        name: "Mini Office Spreadsheet (Amiga)",
        extensions: &["spr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x4E, 0x49, 0x20, 0x4F, 0x46, 0x46, 0x49, 0x43, 0x45, 0x20,
                        0x41, 0x4D, 0x49, 0x47, 0x41, 0x20, 0x2D, 0x20, 0x53, 0x50, 0x52, 0x45,
                        0x41, 0x44, 0x53, 0x48, 0x45, 0x45, 0x54, 0x20, 0x44, 0x41, 0x54, 0x41,
                        0x20, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
