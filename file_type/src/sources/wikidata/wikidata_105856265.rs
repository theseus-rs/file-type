use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856265: FileFormat = FileFormat {
    id: 105_856_265,
    puid: "wikidata/105856265",
    name: "WillDraw Drawing",
    extensions: &["drw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
