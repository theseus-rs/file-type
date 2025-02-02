use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865784: FileFormat = FileFormat {
    id: 105_865_784,
    source_type: SourceType::Wikidata,
    name: "Yahoo! Voice Mail",
    extensions: &["post"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x21, 0x56, 0x4D, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
