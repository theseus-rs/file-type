use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853146: FileFormat = FileFormat {
    id: 105_853_146,
    puid: "wikidata/105853146",
    name: "Moebius Sound Library",
    extensions: &["slb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4C, 0x42, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
