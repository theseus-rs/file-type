use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857447: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_447,
        source_type: SourceType::Wikidata,
        name: "4D Paint Project",
        extensions: &["4dp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x34, 0x44, 0x20, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x20, 0x50, 0x72, 0x6F,
                        0x6A, 0x65, 0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
