use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853048: FileFormat = FileFormat {
    id: 105_853_048,
    source_type: SourceType::Wikidata,
    name: "PRO100 Furniture design project",
    extensions: &["sto"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2A, 0x44, 0x3F, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
