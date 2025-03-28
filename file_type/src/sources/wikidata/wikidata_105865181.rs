use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865181: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_181,
        source_type: SourceType::Wikidata,
        name: "CH Products Pro Throttle Configuration",
        extensions: &["ptc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x0F, 0x50, 0x54, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72,
                        0x61, 0x74, 0x69, 0x6F, 0x6E, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x6F,
                        0x54, 0x68, 0x72, 0x6F, 0x74, 0x74, 0x6C, 0x65, 0x20, 0x56, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
