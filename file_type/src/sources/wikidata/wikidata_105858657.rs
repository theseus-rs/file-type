use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858657: FileFormat = FileFormat {
    id: 105_858_657,
    puid: "wikidata/105858657",
    name: "Crack Art bitmap (low-res)",
    extensions: &["ca1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
