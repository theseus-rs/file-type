use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856921: FileFormat = FileFormat {
    id: 105_856_921,
    puid: "wikidata/105856921",
    name: "The Games Factory Game (G)",
    extensions: &["gam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x41, 0x50, 0x50, 0x07, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
