use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862352: FileFormat = FileFormat {
    id: 105_862_352,
    puid: "wikidata/105862352",
    name: "Mobiclip for Nintendo CTR",
    extensions: &["moflex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x32, 0xAA, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
