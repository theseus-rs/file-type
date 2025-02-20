use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_874,
        source_type: SourceType::Wikidata,
        name: "VistaPro Digital Elevation Map (v4)",
        extensions: &["v4s"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x50, 0x34, 0x20, 0x44, 0x45, 0x4D, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
