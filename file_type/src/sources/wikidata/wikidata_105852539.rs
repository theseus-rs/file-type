use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852539: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_539,
        source_type: SourceType::Wikidata,
        name: "TiEmu Skin (v2.00)",
        extensions: &["skn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x69, 0x45, 0x6D, 0x75, 0x20, 0x76, 0x32, 0x2E, 0x30, 0x30, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0xBE, 0xBA, 0xED, 0xFE,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
