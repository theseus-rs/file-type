use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856069: FileFormat = FileFormat {
    id: 105_856_069,
    puid: "wikidata/105856069",
    name: "Dac-Easy Word Document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x44, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
