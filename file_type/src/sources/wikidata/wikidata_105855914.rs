use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855914: FileFormat = FileFormat {
    id: 105_855_914,
    puid: "wikidata/105855914",
    name: "DFF format (v2.0, BE)",
    extensions: &["dff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x44, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
