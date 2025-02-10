use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852963: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_963,
        source_type: SourceType::Wikidata,
        name: "SpeedTree format",
        extensions: &["spt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xE8, 0x03, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x5F, 0x5F, 0x49, 0x64,
                        0x76, 0x53, 0x70, 0x74, 0x5F, 0x30, 0x32, 0x5F, 0xEA, 0x03, 0x00, 0x00,
                        0xD0, 0x07, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
