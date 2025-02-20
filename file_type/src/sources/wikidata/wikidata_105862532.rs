use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862532: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_532,
        source_type: SourceType::Wikidata,
        name: "Creative Music System Intelligent Organ music",
        extensions: &["org"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4D, 0x53, 0x20, 0x4F, 0x72, 0x67, 0x61, 0x6E, 0x20, 0x44, 0x61,
                        0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x00, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
