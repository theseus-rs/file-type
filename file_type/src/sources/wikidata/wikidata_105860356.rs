use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860356: FileFormat = FileFormat {
    id: 105_860_356,
    puid: "wikidata/105860356",
    name: "RED files library",
    extensions: &["red"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x52, 0x01, 0x29])],
            },
        }],
    }],
    related_formats: &[],
};
