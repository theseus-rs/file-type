use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857596: FileFormat = FileFormat {
    id: 105_857_596,
    source_type: SourceType::Wikidata,
    name: "Install Creator Pro project",
    extensions: &["iip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
