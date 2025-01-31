use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856841: FileFormat = FileFormat {
    id: 105_856_841,
    puid: "wikidata/105856841",
    name: "GPU Friendly Graphics format",
    extensions: &["gfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x46, 0x47, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
