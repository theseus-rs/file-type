use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855980: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_980,
        source_type: SourceType::Wikidata,
        name: "Dynamic Env. data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x20, 0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20, 0x44, 0x65,
                        0x73, 0x6B, 0x20, 0x29, 0x28, 0x20, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E,
                        0x20, 0x29,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
