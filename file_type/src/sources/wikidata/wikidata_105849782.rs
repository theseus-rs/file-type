use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849782: FileFormat = FileFormat {
    id: 105_849_782,
    source_type: SourceType::Wikidata,
    name: "Circuit Diagram Document",
    extensions: &["cddx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x41, 0x67, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
