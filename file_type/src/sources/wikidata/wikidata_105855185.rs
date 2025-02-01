use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855185: FileFormat = FileFormat {
    id: 105_855_185,
    puid: "wikidata/105855185",
    name: "Algor FEMPRO model",
    extensions: &["fem"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x45, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
