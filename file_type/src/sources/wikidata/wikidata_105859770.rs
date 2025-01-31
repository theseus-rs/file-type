use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859770: FileFormat = FileFormat {
    id: 105_859_770,
    puid: "wikidata/105859770",
    name: "MTV video",
    extensions: &["mtv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
