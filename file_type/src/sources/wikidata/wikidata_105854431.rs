use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854431: FileFormat = FileFormat {
    id: 105_854_431,
    puid: "wikidata/105854431",
    name: "AY Amadeus chiptune",
    extensions: &["amad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x41, 0x59, 0x41, 0x4D, 0x41, 0x44, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
