use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851558: FileFormat = FileFormat {
    id: 105_851_558,
    puid: "wikidata/105851558",
    name: "Aladdin 4D TXList",
    extensions: &["txl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x42, 0x4D, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
