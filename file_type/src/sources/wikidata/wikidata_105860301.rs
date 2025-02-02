use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860301: FileFormat = FileFormat {
    id: 105_860_301,
    source_type: SourceType::Wikidata,
    name: "Reason song",
    extensions: &["reason"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x58, 0x31, 0x35, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
