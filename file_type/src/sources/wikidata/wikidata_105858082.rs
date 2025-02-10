use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858082: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_082,
        source_type: SourceType::Wikidata,
        name: "WinAPE IDE hard disk image",
        extensions: &["ide"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x54, 0x57, 0x49, 0x53, 0x45, 0x20, 0x44, 0x52, 0x49, 0x56,
                        0x45, 0x20, 0x49, 0x4D, 0x41, 0x47, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
