use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_888,
        source_type: SourceType::Wikidata,
        name: "BORGChat configuration",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x42, 0x4F, 0x52, 0x47, 0x43, 0x68, 0x61, 0x74, 0x3E, 0x0D, 0x0A,
                        0x20, 0x20, 0x3C, 0x53, 0x65, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x3E, 0x0D,
                        0x0A, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x43, 0x75, 0x72, 0x65, 0x6E, 0x74,
                        0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
