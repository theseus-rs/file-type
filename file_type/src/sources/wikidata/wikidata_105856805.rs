use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856805: FileFormat = FileFormat {
    id: 105_856_805,
    puid: "wikidata/105856805",
    name: "GEM Graph (v1.0)",
    extensions: &["grf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2B, 0x30, 0x37, 0x30, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
