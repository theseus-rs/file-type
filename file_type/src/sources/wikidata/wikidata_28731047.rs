use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28731047: FileFormat = FileFormat {
    id: 28_731_047,
    source_type: SourceType::Wikidata,
    name: "Dyalog APL workspace",
    extensions: &["dws"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
