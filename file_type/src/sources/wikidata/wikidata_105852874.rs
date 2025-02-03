use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852874: FileFormat = FileFormat {
    id: 105_852_874,
    source_type: SourceType::Wikidata,
    name: "Punto 13 System",
    extensions: &["sys"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x67, 0x61, 0x20, 0x53, 0x69, 0x73, 0x74, 0x65, 0x6D, 0x6F, 0x6E,
                    0x65, 0x20, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
