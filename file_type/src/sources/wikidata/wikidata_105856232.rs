use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856232: FileFormat = FileFormat {
    id: 105_856_232,
    puid: "wikidata/105856232",
    name: "Battlefield 2 map Description",
    extensions: &["desc"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x6D, 0x61, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
