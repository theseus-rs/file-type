use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854984: FileFormat = FileFormat {
    id: 105_854_984,
    source_type: SourceType::Wikidata,
    name: "ADX2 HCA Audio",
    extensions: &["awb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x46, 0x53, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
