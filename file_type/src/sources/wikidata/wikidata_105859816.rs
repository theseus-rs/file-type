use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_816,
        source_type: SourceType::Wikidata,
        name: "Surpass Spreadsheet",
        extensions: &["vts"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x09, 0x04, 0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0xF7, 0x7E,
                        0x18, 0x00, 0x92, 0x00, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
