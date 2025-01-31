use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854226: FileFormat = FileFormat {
    id: 105_854_226,
    puid: "wikidata/105854226",
    name: "AMOS Banks group",
    extensions: &["abs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x6D, 0x42, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
