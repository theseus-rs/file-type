use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858784: FileFormat = FileFormat {
    id: 105_858_784,
    puid: "wikidata/105858784",
    name: "Blue Scan drawing",
    extensions: &["blsc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4C, 0x53, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
