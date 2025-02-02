use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858573: FileFormat = FileFormat {
    id: 105_858_573,
    source_type: SourceType::Wikidata,
    name: "BIF bitmap ASCII info",
    extensions: &["bif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
