use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56827096: FileFormat = FileFormat {
    id: 56_827_096,
    puid: "wikidata/56827096",
    name: "Web Assembly Binary Format",
    extensions: &["wasm", "wasm"],
    media_types: &["application/octet-stream", "application/wasm"],
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
