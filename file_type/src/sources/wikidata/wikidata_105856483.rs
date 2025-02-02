use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856483: FileFormat = FileFormat {
    id: 105_856_483,
    source_type: SourceType::Wikidata,
    name: "Westwood game data Archive",
    extensions: &["war"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x41, 0x52, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
