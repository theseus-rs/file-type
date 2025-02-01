use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858092: FileFormat = FileFormat {
    id: 105_858_092,
    puid: "wikidata/105858092",
    name: "IMP eBook (v1.5)",
    extensions: &["imp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x02, 0x00, 0x42, 0x4F, 0x4F, 0x4B, 0x44, 0x4F, 0x55, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
