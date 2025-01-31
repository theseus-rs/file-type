use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856608: FileFormat = FileFormat {
    id: 105_856_608,
    puid: "wikidata/105856608",
    name: "Trilo Tracker Waveform",
    extensions: &["ws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
