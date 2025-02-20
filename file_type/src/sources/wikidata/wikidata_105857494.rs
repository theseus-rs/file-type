use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857494: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_494,
        source_type: SourceType::Wikidata,
        name: "3D Construction Kit 2 Object",
        extensions: &["3od"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6E, 0x20, 0x4F, 0x42, 0x4A, 0x45, 0x43, 0x54, 0x20, 0x66, 0x69,
                        0x6C, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
                        0x69, 0x74, 0x68, 0x20, 0x33, 0x64, 0x20, 0x43, 0x6F, 0x6E, 0x73, 0x74,
                        0x72, 0x75, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x4B, 0x69, 0x74, 0x20,
                        0x32, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
