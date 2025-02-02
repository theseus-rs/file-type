use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853281: FileFormat = FileFormat {
    id: 105_853_281,
    source_type: SourceType::Wikidata,
    name: "Playmation Spin",
    extensions: &["sp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x49, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
