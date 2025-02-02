use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859049: FileFormat = FileFormat {
    id: 105_859_049,
    source_type: SourceType::Wikidata,
    name: "BMW TIS grayscale bitmap",
    extensions: &["itw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x54, 0x57, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
