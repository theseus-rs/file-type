use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855176: FileFormat = FileFormat {
    id: 105_855_176,
    puid: "wikidata/105855176",
    name: "Mortal Kombat 3 game data archive",
    extensions: &["ftr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x46, 0x49, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
