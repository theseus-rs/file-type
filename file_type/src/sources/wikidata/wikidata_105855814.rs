use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855814: FileFormat = FileFormat {
    id: 105_855_814,
    puid: "wikidata/105855814",
    name: "The Graphics Studio patterns",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x53, 0x50, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
