use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850788: FileFormat = FileFormat {
    id: 105_850_788,
    puid: "wikidata/105850788",
    name: "Reflections camera",
    extensions: &["kam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x4B, 0x00, 0x41, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
