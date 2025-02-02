use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865820: FileFormat = FileFormat {
    id: 105_865_820,
    source_type: SourceType::Wikidata,
    name: "PTGui project (old)",
    extensions: &["pts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x70, 0x74, 0x47, 0x75, 0x69, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65,
                    0x63, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
