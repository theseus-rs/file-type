use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857837: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_837,
        source_type: SourceType::Wikidata,
        name: "Call of Duty Modern Warfare 3 DLC",
        extensions: &["iw5dlc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x61, 0x6D, 0x65, 0x3A, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E,
                        0x74, 0x20, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                        0x20, 0x23,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
