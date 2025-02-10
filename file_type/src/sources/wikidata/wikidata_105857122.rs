use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_122,
        source_type: SourceType::Wikidata,
        name: "WinGraph Graph",
        extensions: &["grf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x4D, 0x4A, 0x4D, 0x20, 0x11, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x42, 0x11, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
