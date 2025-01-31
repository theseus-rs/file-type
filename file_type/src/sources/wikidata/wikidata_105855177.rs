use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855177: FileFormat = FileFormat {
    id: 105_855_177,
    puid: "wikidata/105855177",
    name: "FASA Interactive game data archive",
    extensions: &["fst"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAF, 0xEC, 0xDD, 0xCA])],
            },
        }],
    }],
    related_formats: &[],
};
