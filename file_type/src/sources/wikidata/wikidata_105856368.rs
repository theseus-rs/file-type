use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856368: FileFormat = FileFormat {
    id: 105_856_368,
    source_type: SourceType::Wikidata,
    name: "Symantec QandA Database File",
    extensions: &["dtf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x4D, 0x41, 0x4E, 0x54, 0x45, 0x43, 0x20, 0x54, 0x4F, 0x55, 0x43,
                    0x48, 0x42, 0x41, 0x53, 0x45, 0x20, 0x44, 0x41, 0x54, 0x41, 0x42, 0x41, 0x53,
                    0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
