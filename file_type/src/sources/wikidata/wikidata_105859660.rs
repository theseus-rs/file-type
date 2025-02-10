use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859660: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_660,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual Studio Solution v10.00/2008 (CRLF)",
        extensions: &["sln"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x0D, 0x0A, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F,
                        0x66, 0x74, 0x20, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6C, 0x20, 0x53, 0x74,
                        0x75, 0x64, 0x69, 0x6F, 0x20, 0x53, 0x6F, 0x6C, 0x75, 0x74, 0x69, 0x6F,
                        0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2C, 0x20, 0x46, 0x6F, 0x72, 0x6D,
                        0x61, 0x74, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                        0x30, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
