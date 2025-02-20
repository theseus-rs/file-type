use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51923000: FileType = FileType {
    file_format: &FileFormat {
        id: 51_923_000,
        source_type: SourceType::Wikidata,
        name: "Applixware Spreadsheet",
        extensions: &["as"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x53, 0x50, 0x52, 0x45, 0x41,
                        0x44, 0x53, 0x48, 0x45, 0x45, 0x54, 0x53, 0x20, 0x56, 0x45, 0x52, 0x53,
                        0x49, 0x4F, 0x4E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
