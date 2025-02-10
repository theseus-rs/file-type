use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_601,
        source_type: SourceType::Wikidata,
        name: "OpenTimestamps proof",
        extensions: &["ots"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x4F, 0x70, 0x65, 0x6E, 0x54, 0x69, 0x6D, 0x65, 0x73, 0x74, 0x61,
                        0x6D, 0x70, 0x73, 0x00, 0x00, 0x50, 0x72, 0x6F, 0x6F, 0x66,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
