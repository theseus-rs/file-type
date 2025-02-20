use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860071: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_071,
        source_type: SourceType::Wikidata,
        name: "Packed Animation File video",
        extensions: &["paf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x63, 0x6B, 0x65, 0x64, 0x20, 0x41, 0x6E, 0x69, 0x6D, 0x61,
                        0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x31,
                        0x2E, 0x30, 0x0A, 0x28, 0x63, 0x29, 0x20, 0x31, 0x39, 0x39, 0x32, 0x2D,
                        0x39, 0x36, 0x20, 0x41, 0x6D, 0x61, 0x7A, 0x69, 0x6E, 0x67, 0x20, 0x53,
                        0x74, 0x75, 0x64, 0x69, 0x6F, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
