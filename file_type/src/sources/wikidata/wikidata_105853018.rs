use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853018: FileFormat = FileFormat {
    id: 105_853_018,
    source_type: SourceType::Wikidata,
    name: "Circuit Maker chart",
    extensions: &["ssf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x43, 0x68, 0x61, 0x72, 0x74, 0x53, 0x65, 0x74, 0x75, 0x70, 0x73, 0x42,
                    0x65, 0x67, 0x69, 0x6E, 0x0A, 0x43, 0x68, 0x61, 0x72, 0x74, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
