use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850502: FileFormat = FileFormat {
    id: 105_850_502,
    source_type: SourceType::Wikidata,
    name: "OCaml bytecode (library)",
    extensions: &["cma"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x6D, 0x6C, 0x31, 0x39, 0x39, 0x39, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
