use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866392: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_392,
        source_type: SourceType::Wikidata,
        name: "PageSetter III page",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x12, 0x19, 0x18, 0x17, 0x50, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74,
                        0x65, 0x72, 0xB3, 0x20, 0xA9, 0x31, 0x39, 0x39, 0x32, 0x20, 0x47, 0x6F,
                        0x6C, 0x64, 0x20, 0x44, 0x69, 0x73, 0x6B, 0x20, 0x49, 0x6E, 0x63, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
