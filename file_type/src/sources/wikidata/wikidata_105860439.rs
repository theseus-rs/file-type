use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860439: FileFormat = FileFormat {
    id: 105_860_439,
    source_type: SourceType::Wikidata,
    name: "Okino plugin Run Time Information",
    extensions: &["rti"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
