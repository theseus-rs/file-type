use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855763: FileFormat = FileFormat {
    id: 105_855_763,
    puid: "wikidata/105855763",
    name: "Battlefield 2 mod Description",
    extensions: &["desc"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x6D, 0x6F, 0x64, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
