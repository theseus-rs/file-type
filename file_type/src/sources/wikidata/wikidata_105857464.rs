use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857464: FileFormat = FileFormat {
    id: 105_857_464,
    source_type: SourceType::Wikidata,
    name: "Aladdin 4D Drawing (generic)",
    extensions: &["4d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
