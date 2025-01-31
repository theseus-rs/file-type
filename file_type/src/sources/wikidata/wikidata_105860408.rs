use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860408: FileFormat = FileFormat {
    id: 105_860_408,
    puid: "wikidata/105860408",
    name: "REKO cardset",
    extensions: &["reko"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x4B, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
