use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855082: FileFormat = FileFormat {
    id: 105_855_082,
    puid: "wikidata/105855082",
    name: "L3DT Attributes Map File",
    extensions: &["amf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x33, 0x44, 0x54, 0x08, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
