use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865199: FileFormat = FileFormat {
    id: 105_865_199,
    source_type: SourceType::Wikidata,
    name: "ClickArt Personal Publisher document",
    extensions: &["pub"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0xAA, 0xAA, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
