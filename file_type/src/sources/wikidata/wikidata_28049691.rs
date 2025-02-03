use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049691: FileFormat = FileFormat {
    id: 28_049_691,
    source_type: SourceType::Wikidata,
    name: "BRL-CAD geometry",
    extensions: &["g"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x35, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
