use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860746: FileFormat = FileFormat {
    id: 105_860_746,
    puid: "wikidata/105860746",
    name: "ReSource Control Language",
    extensions: &["rcl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
