use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858807: FileFormat = FileFormat {
    id: 105_858_807,
    source_type: SourceType::Wikidata,
    name: "Ipaint bitmap",
    extensions: &["ip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x42, 0x52, 0x55, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
