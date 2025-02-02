use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857339: FileFormat = FileFormat {
    id: 105_857_339,
    source_type: SourceType::Wikidata,
    name: "Embedded JCL debug info",
    extensions: &["jdbg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x44, 0x42, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
