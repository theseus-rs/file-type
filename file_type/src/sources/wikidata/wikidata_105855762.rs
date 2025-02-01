use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855762: FileFormat = FileFormat {
    id: 105_855_762,
    puid: "wikidata/105855762",
    name: "DFF format (v2.0, LE)",
    extensions: &["dff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x46, 0x44, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
