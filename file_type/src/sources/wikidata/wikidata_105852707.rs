use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852707: FileFormat = FileFormat {
    id: 105_852_707,
    puid: "wikidata/105852707",
    name: "Sprite Pad Data (v2.0)",
    extensions: &["spd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x44, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
