use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857686: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_686,
        source_type: SourceType::Wikidata,
        name: "IBM Planning Assistant spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6C, 0x61, 0x6E, 0x20, 0x20, 0x01, 0x00, 0x03, 0x41, 0x30, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
