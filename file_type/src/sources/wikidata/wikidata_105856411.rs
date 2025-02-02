use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856411: FileFormat = FileFormat {
    id: 105_856_411,
    source_type: SourceType::Wikidata,
    name: "Words and Figures document",
    extensions: &["waf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x6F, 0x72, 0x64, 0x73, 0x20, 0x26, 0x20, 0x46, 0x69, 0x67, 0x75, 0x72,
                    0x65, 0x73, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
