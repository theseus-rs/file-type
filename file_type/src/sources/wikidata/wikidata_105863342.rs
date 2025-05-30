use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_342,
        source_type: SourceType::Wikidata,
        name: "MetaMind Machine Sequence",
        extensions: &["mmms"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x53, 0x65, 0x65, 0x20, 0x68, 0x74, 0x74,
                        0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6D, 0x65, 0x74, 0x61,
                        0x2D, 0x6D, 0x69, 0x6E, 0x64, 0x2E, 0x64, 0x65, 0x20, 0x2D, 0x2D, 0x3E,
                        0x0A, 0x0A, 0x3C, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x63, 0x65, 0x3E,
                        0x0A, 0x3C, 0x6D, 0x65, 0x74, 0x61, 0x6D, 0x69, 0x6E, 0x64, 0x6D, 0x61,
                        0x63, 0x68, 0x69, 0x6E, 0x65, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
