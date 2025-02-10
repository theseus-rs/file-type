use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856607: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_607,
        source_type: SourceType::Wikidata,
        name: "Chalk game data archive",
        extensions: &["wgm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x47, 0x4D, 0x31, 0x01, 0x00, 0x53, 0x79, 0x6E, 0x74, 0x65, 0x73,
                        0x69, 0x73, 0x20, 0x45, 0x6E, 0x74, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6E,
                        0x6D, 0x65, 0x6E, 0x74, 0x3A, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F,
                        0x2F, 0x77, 0x73, 0x66, 0x2E, 0x73, 0x79, 0x6E, 0x74, 0x65, 0x73, 0x69,
                        0x73, 0x2E, 0x61, 0x74, 0x68, 0x2E, 0x63, 0x78, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
