use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856698: FileFormat = FileFormat {
    id: 105_856_698,
    source_type: SourceType::Wikidata,
    name: "Universal Data Format (lower case)",
    extensions: &["udf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x24, 0x64, 0x6F, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
