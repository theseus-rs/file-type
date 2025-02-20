use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862910: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_910,
        source_type: SourceType::Wikidata,
        name: "MMFW Blobs",
        extensions: &["mmb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x46, 0x57, 0x20, 0x42, 0x6C, 0x6F, 0x62, 0x73, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x4D, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
