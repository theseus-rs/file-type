use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857707: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_707,
        source_type: SourceType::Wikidata,
        name: "North Star Horizon Hard Disk image",
        extensions: &["nhd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0xFF, 0x54, 0x52, 0x41, 0x4E, 0x53, 0x49, 0x45, 0x4E, 0x54, 0x2C,
                        0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x2C, 0x31, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
