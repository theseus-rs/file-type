use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858483: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_483,
        source_type: SourceType::Wikidata,
        name: "Sunplus Burn firmware update",
        extensions: &["brn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x55, 0x4E, 0x50, 0x20, 0x42, 0x55, 0x52, 0x4E, 0x20, 0x46, 0x49,
                        0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
