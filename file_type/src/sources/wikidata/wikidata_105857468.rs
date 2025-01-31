use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857468: FileFormat = FileFormat {
    id: 105_857_468,
    puid: "wikidata/105857468",
    name: "5View capture",
    extensions: &["5vw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0xAA, 0xAA, 0xAA])],
            },
        }],
    }],
    related_formats: &[],
};
