use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853363: FileFormat = FileFormat {
    id: 105_853_363,
    puid: "wikidata/105853363",
    name: "Snzip compressed (snappy-java format)",
    extensions: &["snappy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x82, 0x53, 0x4E, 0x41, 0x50, 0x50, 0x59])],
            },
        }],
    }],
    related_formats: &[],
};
