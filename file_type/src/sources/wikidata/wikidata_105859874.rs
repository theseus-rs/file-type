use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859874: FileFormat = FileFormat {
    id: 105_859_874,
    puid: "wikidata/105859874",
    name: "VistaPro Digital Elevation Map (v4)",
    extensions: &["v4s"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
};
