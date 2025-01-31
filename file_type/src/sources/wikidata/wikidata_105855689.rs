use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855689: FileFormat = FileFormat {
    id: 105_855_689,
    puid: "wikidata/105855689",
    name: "OCAD map",
    extensions: &["ocd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAD, 0x0C])],
            },
        }],
    }],
    related_formats: &[],
};
