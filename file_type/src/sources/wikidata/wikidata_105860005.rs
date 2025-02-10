use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860005: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_005,
        source_type: SourceType::Wikidata,
        name: "Red Sector Demo-Maker vector object",
        extensions: &["vec"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x45, 0x43, 0x54, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
