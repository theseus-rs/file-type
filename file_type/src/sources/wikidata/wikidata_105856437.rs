use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856437: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_437,
        source_type: SourceType::Wikidata,
        name: "Wind4Deap 2 project",
        extensions: &["wdjson"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x20, 0x22, 0x4D, 0x65, 0x74, 0x61, 0x20, 0x64, 0x61, 0x74, 0x61,
                        0x22, 0x20, 0x3A, 0x20, 0x7B, 0x20, 0x22, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x74, 0x79, 0x70, 0x65, 0x22, 0x20, 0x3A, 0x20, 0x22, 0x57, 0x69, 0x6E,
                        0x34, 0x44, 0x65, 0x61, 0x70, 0x20, 0x6A, 0x73, 0x6F, 0x6E, 0x22, 0x2C,
                        0x20, 0x22, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x20, 0x3A,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
