use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860547: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_547,
        source_type: SourceType::Wikidata,
        name: "MicroTik RouterOS debugging / support dump",
        extensions: &["rif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x52, 0x4F, 0x55, 0x54,
                        0x45, 0x52, 0x4F, 0x53, 0x20, 0x53, 0x55, 0x50, 0x4F, 0x55, 0x54, 0x20,
                        0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
