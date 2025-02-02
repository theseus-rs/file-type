use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206412: FileFormat = FileFormat {
    id: 28_206_412,
    source_type: SourceType::Wikidata,
    name: "Jovian Logic VI",
    extensions: &["vi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
