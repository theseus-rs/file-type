use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855790: FileFormat = FileFormat {
    id: 105_855_790,
    puid: "wikidata/105855790",
    name: "LocoScript Data (v1.x)",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x54, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
