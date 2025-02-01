use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859207: FileFormat = FileFormat {
    id: 105_859_207,
    puid: "wikidata/105859207",
    name: "Axon Raw Format bitmap (big endian)",
    extensions: &["arf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x41, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
