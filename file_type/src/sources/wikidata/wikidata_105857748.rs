use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857748: FileFormat = FileFormat {
    id: 105_857_748,
    source_type: SourceType::Wikidata,
    name: "IMP eBook (v1.0)",
    extensions: &["imp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x42, 0x4F, 0x4F, 0x4B, 0x44, 0x4F, 0x55, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
