use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858717: FileFormat = FileFormat {
    id: 105_858_717,
    puid: "wikidata/105858717",
    name: "BOLT game data archive",
    extensions: &["blt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x4C, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
