use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852953: FileFormat = FileFormat {
    id: 105_852_953,
    source_type: SourceType::Wikidata,
    name: "Spring Map Definition",
    extensions: &["smd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4D, 0x41, 0x50, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
