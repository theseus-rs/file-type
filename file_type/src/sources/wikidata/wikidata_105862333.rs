use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862333: FileFormat = FileFormat {
    id: 105_862_333,
    source_type: SourceType::Wikidata,
    name: "XMOD format module",
    extensions: &["xmod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x4D, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
