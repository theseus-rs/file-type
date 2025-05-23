use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856520: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_520,
        source_type: SourceType::Wikidata,
        name: "WinWorks spreadsheet",
        extensions: &["wpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20, 0x53, 0x70, 0x72,
                        0x65, 0x61, 0x64, 0x53, 0x68, 0x65, 0x65, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
