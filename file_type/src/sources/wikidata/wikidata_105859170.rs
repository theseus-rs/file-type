use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859170: FileFormat = FileFormat {
    id: 105_859_170,
    puid: "wikidata/105859170",
    name: "Mallard BASIC tokenized source (v1.11)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFC, 0x03, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
