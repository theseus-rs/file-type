use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857001: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_001,
        source_type: SourceType::Wikidata,
        name: "ElmerGrid input data (var 2)",
        extensions: &["grd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x45, 0x6C, 0x6D, 0x65, 0x72, 0x47,
                        0x72, 0x69, 0x64, 0x20, 0x69, 0x6E, 0x70, 0x75, 0x74, 0x20, 0x66, 0x69,
                        0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63,
                        0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x67, 0x72, 0x69, 0x64, 0x20, 0x67,
                        0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x2A, 0x2A,
                        0x2A, 0x2A, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
