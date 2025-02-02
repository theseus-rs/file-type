use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860719: FileFormat = FileFormat {
    id: 105_860_719,
    source_type: SourceType::Wikidata,
    name: "Office Live Meeting Connection",
    extensions: &["rtc"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x72, 0x74, 0x63, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
