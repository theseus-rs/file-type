use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855471: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_471,
        source_type: SourceType::Wikidata,
        name: "Full Impact spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x52, 0x4B, 0x52, 0x57, 0x57, 0x45, 0x47, 0x52, 0x50, 0x41, 0x4D,
                        0x44, 0x47, 0x42, 0x4B, 0x52, 0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
