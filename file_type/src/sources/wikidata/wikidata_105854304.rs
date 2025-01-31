use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854304: FileFormat = FileFormat {
    id: 105_854_304,
    puid: "wikidata/105854304",
    name: "HAL Laboratory HALPST container audio",
    extensions: &["hps"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x48, 0x41, 0x4C, 0x50, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
