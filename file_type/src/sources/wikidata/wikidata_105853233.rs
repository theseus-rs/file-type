use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853233: FileFormat = FileFormat {
    id: 105_853_233,
    source_type: SourceType::Wikidata,
    name: "Windows Shadow spooler (98)",
    extensions: &["shd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x49, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
