use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860420: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_420,
        source_type: SourceType::Wikidata,
        name: "Klasik Resource data",
        extensions: &["res"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x72, 0x65, 0x73, 0x6F, 0x75,
                        0x72, 0x63, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
