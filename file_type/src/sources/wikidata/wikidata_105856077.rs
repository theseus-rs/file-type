use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856077: FileFormat = FileFormat {
    id: 105_856_077,
    puid: "wikidata/105856077",
    name: "Graph Diagram",
    extensions: &["dia"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x72, 0x61, 0x70, 0x68, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
