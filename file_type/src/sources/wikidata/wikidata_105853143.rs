use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853143: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_143,
        source_type: SourceType::Wikidata,
        name: "K-Spreadsheet (v3/4)",
        extensions: &["spd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4C, 0x49, 0x46, 0x46, 0x20, 0x48, 0x61, 0x72, 0x6B, 0x65, 0x72,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
