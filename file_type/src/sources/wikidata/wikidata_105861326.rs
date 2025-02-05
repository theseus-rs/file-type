use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861326: FileFormat = FileFormat {
    id: 105_861_326,
    source_type: SourceType::Wikidata,
    name: "64LAN container",
    extensions: &["l64"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x36, 0x34, 0x4C, 0x41, 0x4E, 0x20, 0x49, 0x44, 0x42, 0x4C, 0x4F, 0x43, 0x4B,
                    0x20, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
