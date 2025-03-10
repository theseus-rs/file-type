use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852188: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_188,
        source_type: SourceType::Wikidata,
        name: "BiPlane spreadsheet",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x44, 0x3B, 0x50, 0x42, 0x49, 0x50, 0x4C, 0x3B, 0x4E, 0x3B, 0x52,
                        0x3B, 0x41, 0x0D, 0x52, 0x45, 0x4D, 0x20, 0x42, 0x69, 0x50, 0x6C, 0x61,
                        0x6E, 0x65, 0x20, 0x56, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
