use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855486: FileFormat = FileFormat {
    id: 105_855_486,
    puid: "wikidata/105855486",
    name: "The Need For Speed Font",
    extensions: &["ffn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4E, 0x54, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
