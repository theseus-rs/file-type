use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_732,
        source_type: SourceType::Wikidata,
        name: "Windows Registry Data (Ver. 5.0)",
        extensions: &["reg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x52, 0x65, 0x67, 0x69,
                        0x73, 0x74, 0x72, 0x79, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x35, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
