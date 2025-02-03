use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_20155677: FileFormat = FileFormat {
    id: 20_155_677,
    source_type: SourceType::Wikidata,
    name: "WebAssembly",
    extensions: &["wasm", "wast"],
    media_types: &["application/wasm"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x61, 0x73, 0x6D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x61, 0x73, 0x6D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
