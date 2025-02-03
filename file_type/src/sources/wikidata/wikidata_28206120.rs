use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206120: FileFormat = FileFormat {
    id: 28_206_120,
    source_type: SourceType::Wikidata,
    name: "Flickering Flexible Line Interpratation",
    extensions: &["ffli"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x3A, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
