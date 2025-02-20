use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864856: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_856,
        source_type: SourceType::Wikidata,
        name: "Call Of Duty: Finest Hour game data archive",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x61, 0x72, 0x6B, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x20, 0x28,
                        0x43, 0x29, 0x32, 0x30, 0x30, 0x34, 0x20, 0x53, 0x70, 0x61, 0x72, 0x6B,
                        0x20, 0x55, 0x6E, 0x6C, 0x69, 0x6D, 0x69, 0x74, 0x65, 0x64, 0x2C, 0x20,
                        0x49, 0x6E, 0x63, 0x2E, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x20,
                        0x4A, 0x69, 0x6D, 0x20, 0x53, 0x63, 0x68, 0x75, 0x6C, 0x65, 0x72, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
