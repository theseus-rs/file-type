use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852630: FileFormat = FileFormat {
    id: 105_852_630,
    source_type: SourceType::Wikidata,
    name: "gEDA Symbol",
    extensions: &["sym"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x20, 0x32, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
