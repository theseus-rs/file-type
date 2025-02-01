use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858389: FileFormat = FileFormat {
    id: 105_858_389,
    puid: "wikidata/105858389",
    name: "Envoy document (v1)",
    extensions: &["evy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
