use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_76158553: FileType = FileType {
    file_format: &FileFormat {
        id: 76_158_553,
        source_type: SourceType::Wikidata,
        name: "MegaPaint VIN",
        extensions: &["vin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x56, 0x49, 0x4E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
