use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849727: FileFormat = FileFormat {
    id: 105_849_727,
    source_type: SourceType::Wikidata,
    name: "Canvas document (v5)",
    extensions: &["cv5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x4E, 0x56, 0x41, 0x53, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
