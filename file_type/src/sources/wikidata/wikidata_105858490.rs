use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858490: FileFormat = FileFormat {
    id: 105_858_490,
    puid: "wikidata/105858490",
    name: "BrioQuery",
    extensions: &["bqy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x42, 0x52, 0x49, 0x46, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
