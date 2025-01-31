use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858674: FileFormat = FileFormat {
    id: 105_858_674,
    puid: "wikidata/105858674",
    name: "Axon Raw Format bitmap (little endian)",
    extensions: &["arf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x41, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
