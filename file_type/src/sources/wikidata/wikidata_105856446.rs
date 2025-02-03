use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856446: FileFormat = FileFormat {
    id: 105_856_446,
    source_type: SourceType::Wikidata,
    name: "Windows Sidebar Style",
    extensions: &["wsstyles"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x52, 0x53, 0x54, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
