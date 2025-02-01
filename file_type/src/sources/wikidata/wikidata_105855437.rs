use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855437: FileFormat = FileFormat {
    id: 105_855_437,
    puid: "wikidata/105855437",
    name: "File-Type Rule",
    extensions: &["ftr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x59, 0x50, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
