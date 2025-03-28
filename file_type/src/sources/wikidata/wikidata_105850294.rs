use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850294: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_294,
        source_type: SourceType::Wikidata,
        name: "ColdFusion Template",
        extensions: &["cfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6C, 0x6C, 0x61, 0x69, 0x72, 0x65, 0x20, 0x43, 0x6F, 0x6C, 0x64,
                        0x20, 0x46, 0x75, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x54, 0x65, 0x6D, 0x70,
                        0x6C, 0x61, 0x74, 0x65, 0x0A, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
                        0x53, 0x69, 0x7A, 0x65, 0x3A, 0x20, 0x4E, 0x65, 0x77, 0x20, 0x56, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
