use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864734: FileFormat = FileFormat {
    id: 105_864_734,
    source_type: SourceType::Wikidata,
    name: "Professional Music Driver PVI samples pack (v1)",
    extensions: &["pvi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x49, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
