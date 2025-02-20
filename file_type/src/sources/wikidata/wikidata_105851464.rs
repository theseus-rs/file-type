use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851464: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_464,
        source_type: SourceType::Wikidata,
        name: "Klasik Table/spreadsheet",
        extensions: &["tab"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x74, 0x61, 0x62, 0x75, 0x6C,
                        0x6B, 0x61, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
