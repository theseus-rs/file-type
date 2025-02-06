use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850284: FileFormat = FileFormat {
    id: 105_850_284,
    source_type: SourceType::Wikidata,
    name: "OCaml bytecode (native library)",
    extensions: &["cmxa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x6D, 0x6C, 0x31, 0x39, 0x39, 0x39, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
