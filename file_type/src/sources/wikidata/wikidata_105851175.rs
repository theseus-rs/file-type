use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851175: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_175,
        source_type: SourceType::Wikidata,
        name: "Klasik Text document",
        extensions: &["txk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
