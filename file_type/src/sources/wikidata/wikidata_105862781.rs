use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862781: FileFormat = FileFormat {
    id: 105_862_781,
    source_type: SourceType::Wikidata,
    name: "COMSOL Multiphysics mesh (txt)",
    extensions: &["mphtxt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
