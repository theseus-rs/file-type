use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856067: FileFormat = FileFormat {
    id: 105_856_067,
    puid: "wikidata/105856067",
    name: "Digital Symphony relocatable module",
    extensions: &["dsym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x02, 0x01, 0x13, 0x13, 0x14, 0x12, 0x01, 0x0B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
