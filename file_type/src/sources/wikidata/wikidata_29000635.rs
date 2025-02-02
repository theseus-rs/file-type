use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000635: FileFormat = FileFormat {
    id: 29_000_635,
    source_type: SourceType::Wikidata,
    name: "PolyHedral Database",
    extensions: &["phd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x6E, 0x61, 0x6D, 0x65, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
