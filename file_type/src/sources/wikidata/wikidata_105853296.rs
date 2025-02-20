use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853296: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_296,
        source_type: SourceType::Wikidata,
        name: "Stacker compressed volume (v4.x)",
        extensions: &["000"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x54, 0x41, 0x43, 0x4B, 0x45, 0x52, 0x20, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x20, 0x20, 0x34, 0x20, 0x20, 0x20, 0x20, 0x76,
                        0x6F, 0x6C, 0x75, 0x6D, 0x65, 0x3A, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
