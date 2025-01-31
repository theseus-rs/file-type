use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856317: FileFormat = FileFormat {
    id: 105_856_317,
    puid: "wikidata/105856317",
    name: "Dream Station Instruments",
    extensions: &["dsi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x53, 0x49, 0x6E, 0x73, 0x31, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
