use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76143366: FileFormat = FileFormat {
    id: 76_143_366,
    source_type: SourceType::Wikidata,
    name: "TeX Virtual Font",
    extensions: &["vf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF7, 0xCA])],
            },
        }],
    }],
    related_formats: &[],
};
