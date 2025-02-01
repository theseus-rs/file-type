use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855893: FileFormat = FileFormat {
    id: 105_855_893,
    puid: "wikidata/105855893",
    name: "FL Studio DrumKit (generic)",
    extensions: &["dmkit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
