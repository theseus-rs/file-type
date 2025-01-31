use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854047: FileFormat = FileFormat {
    id: 105_854_047,
    puid: "wikidata/105854047",
    name: "LZUF compressed data",
    extensions: &["lzu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x55, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
