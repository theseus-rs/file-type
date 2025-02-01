use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853120: FileFormat = FileFormat {
    id: 105_853_120,
    puid: "wikidata/105853120",
    name: "Magic and Mayhem sprites",
    extensions: &["spr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x52, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
