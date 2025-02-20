use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852336: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_336,
        source_type: SourceType::Wikidata,
        name: "dBASE IV Screen",
        extensions: &["scr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x42, 0x41, 0x53, 0x45, 0x20, 0x49, 0x56, 0x20, 0x47, 0x65, 0x6E,
                        0x65, 0x72, 0x69, 0x63, 0x20, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x20,
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x20, 0x31, 0x2E, 0x30, 0x00, 0x01, 0x12, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
