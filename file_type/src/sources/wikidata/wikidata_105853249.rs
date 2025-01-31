use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853249: FileFormat = FileFormat {
    id: 105_853_249,
    puid: "wikidata/105853249",
    name: "Live for Speed data",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x46, 0x53, 0x53, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
