use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861927: FileFormat = FileFormat {
    id: 105_861_927,
    puid: "wikidata/105861927",
    name: "Cisco IOS mzip compressed data",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
