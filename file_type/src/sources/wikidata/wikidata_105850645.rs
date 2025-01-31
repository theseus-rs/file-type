use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850645: FileFormat = FileFormat {
    id: 105_850_645,
    puid: "wikidata/105850645",
    name: "Knowledge Master Concept Map",
    extensions: &["kmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x57, 0x04, 0x01, 0x31, 0x12])],
            },
        }],
    }],
    related_formats: &[],
};
