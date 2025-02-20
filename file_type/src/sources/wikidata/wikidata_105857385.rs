use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857385: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_385,
        source_type: SourceType::Wikidata,
        name: "Tripod Data Systems Job file",
        extensions: &["job"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x72, 0x69, 0x70, 0x6F, 0x64, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20,
                        0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x73, 0x20, 0x4A, 0x6F, 0x62, 0x20,
                        0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
