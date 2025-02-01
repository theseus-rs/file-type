use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853270: FileFormat = FileFormat {
    id: 105_853_270,
    puid: "wikidata/105853270",
    name: "Snzip compressed (snappy-in-java format)",
    extensions: &["snappy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x6E, 0x61, 0x70, 0x70, 0x79, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
