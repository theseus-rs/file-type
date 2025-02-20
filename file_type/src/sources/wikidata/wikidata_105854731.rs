use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854731: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_731,
        source_type: SourceType::Wikidata,
        name: "Unity YAML Scene",
        extensions: &["asset"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x59, 0x41, 0x4D, 0x4C, 0x20, 0x31, 0x2E, 0x31, 0x0A, 0x25, 0x54,
                        0x41, 0x47, 0x20, 0x21, 0x75, 0x21, 0x20, 0x74, 0x61, 0x67, 0x3A, 0x75,
                        0x6E, 0x69, 0x74, 0x79, 0x33, 0x64, 0x2E, 0x63, 0x6F, 0x6D, 0x2C, 0x32,
                        0x30, 0x31, 0x31, 0x3A, 0x0A, 0x2D, 0x2D, 0x2D, 0x20, 0x21, 0x75, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
