use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856089: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_089,
        source_type: SourceType::Wikidata,
        name: "Dust3D XML model (v1.0)",
        extensions: &["ds3"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x55, 0x53, 0x54, 0x33, 0x44, 0x20, 0x31, 0x2E, 0x30, 0x20, 0x78,
                        0x6D, 0x6C, 0x20, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
