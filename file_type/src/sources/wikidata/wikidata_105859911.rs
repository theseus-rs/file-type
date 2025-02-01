use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859911: FileFormat = FileFormat {
    id: 105_859_911,
    puid: "wikidata/105859911",
    name: "Garmin Voice Processing Module",
    extensions: &["vpm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x55, 0x44, 0x49, 0x4D, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
