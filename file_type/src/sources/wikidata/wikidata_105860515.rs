use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860515: FileFormat = FileFormat {
    id: 105_860_515,
    source_type: SourceType::Wikidata,
    name: "Windows Registry Data",
    extensions: &["reg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x47, 0x45, 0x44, 0x49, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
