use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853810: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_810,
        source_type: SourceType::Wikidata,
        name: "WinAsks Player compiled app (v2.00)",
        extensions: &["ask"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1C, 0x00, 0x57, 0x49, 0x4E, 0x41, 0x53, 0x4B, 0x53, 0x20, 0x50, 0x4C,
                        0x41, 0x59, 0x45, 0x52, 0x20, 0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x45,
                        0x44, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x04, 0x00, 0x32, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
