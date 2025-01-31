use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853301: FileFormat = FileFormat {
    id: 105_853_301,
    puid: "wikidata/105853301",
    name: "3M Printscape document (v1.x)",
    extensions: &["std"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x56, 0x00, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
