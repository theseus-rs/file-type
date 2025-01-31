use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855708: FileFormat = FileFormat {
    id: 105_855_708,
    puid: "wikidata/105855708",
    name: "Oric Raw Tape format",
    extensions: &["ort"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x52, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
