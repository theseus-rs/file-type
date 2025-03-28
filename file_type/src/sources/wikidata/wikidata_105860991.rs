use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860991: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_991,
        source_type: SourceType::Wikidata,
        name: "Logomotion Graphic File",
        extensions: &["lgf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x47, 0x46, 0x30, 0x34, 0x01, 0x00, 0x00, 0x00, 0x49, 0x4D, 0x5A,
                        0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
