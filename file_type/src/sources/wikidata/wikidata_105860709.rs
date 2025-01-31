use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860709: FileFormat = FileFormat {
    id: 105_860_709,
    puid: "wikidata/105860709",
    name: "Rule of Rose game data archive",
    extensions: &["rpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x54, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
