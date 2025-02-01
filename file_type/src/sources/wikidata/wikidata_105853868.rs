use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853868: FileFormat = FileFormat {
    id: 105_853_868,
    puid: "wikidata/105853868",
    name: "mkwACT lossless compressed audio (v2)",
    extensions: &["mkw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x6B, 0x77, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
