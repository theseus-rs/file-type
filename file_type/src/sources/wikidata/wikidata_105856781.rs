use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856781: FileFormat = FileFormat {
    id: 105_856_781,
    source_type: SourceType::Wikidata,
    name: "High Speed Pascal compiled Unit",
    extensions: &["unit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4E, 0x49, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
