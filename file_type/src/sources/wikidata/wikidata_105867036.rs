use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867036: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_036,
        source_type: SourceType::Wikidata,
        name: "JB BAHN vehicle",
        extensions: &["nfz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x61, 0x74, 0x65, 0x69, 0x20, 0x6D, 0x69, 0x74, 0x20, 0x6E, 0x75,
                        0x74, 0x7A, 0x65, 0x72, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x65, 0x72,
                        0x74, 0x65, 0x6E, 0x20, 0x46, 0x61, 0x68, 0x72, 0x7A, 0x65, 0x75, 0x67,
                        0x65, 0x6E, 0x0D, 0x0A, 0x45, 0x72, 0x73, 0x74, 0x65, 0x6C, 0x6C, 0x74,
                        0x20, 0x6D, 0x69, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
