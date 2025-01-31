use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854041: FileFormat = FileFormat {
    id: 105_854_041,
    puid: "wikidata/105854041",
    name: "Power Up! Album project (v1.x)",
    extensions: &["alb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x77, 0x65, 0x72, 0x55, 0x70, 0x20, 0x41, 0x6C, 0x62, 0x75, 0x6D,
                    0x20, 0x76, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
