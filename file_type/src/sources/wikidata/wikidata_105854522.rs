use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854522: FileFormat = FileFormat {
    id: 105_854_522,
    puid: "wikidata/105854522",
    name: "Arena opening Book",
    extensions: &["abk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x41, 0x42, 0x4B, 0x70, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
