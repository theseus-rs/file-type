use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853132: FileFormat = FileFormat {
    id: 105_853_132,
    puid: "wikidata/105853132",
    name: "Bars and Pipes Professional song",
    extensions: &["song"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x52, 0x50, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
