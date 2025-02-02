use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860911: FileFormat = FileFormat {
    id: 105_860_911,
    source_type: SourceType::Wikidata,
    name: "SpartaDOS X binary image",
    extensions: &["rom"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x44, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
