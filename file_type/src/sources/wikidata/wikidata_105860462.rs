use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860462: FileFormat = FileFormat {
    id: 105_860_462,
    source_type: SourceType::Wikidata,
    name: "R saved work space",
    extensions: &["rdata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x58, 0x32, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
