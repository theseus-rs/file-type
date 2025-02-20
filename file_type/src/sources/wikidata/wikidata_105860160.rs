use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860160: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_160,
        source_type: SourceType::Wikidata,
        name: "FLEXIT Multishot Survey Raw Data file",
        extensions: &["rsy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x4C, 0x45, 0x58, 0x49, 0x54, 0x20, 0x4D, 0x75, 0x6C, 0x74,
                        0x69, 0x73, 0x68, 0x6F, 0x74, 0x20, 0x53, 0x75, 0x72, 0x76, 0x65, 0x79,
                        0x20, 0x52, 0x61, 0x77, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x3A, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
