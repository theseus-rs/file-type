use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851458: FileFormat = FileFormat {
    id: 105_851_458,
    puid: "wikidata/105851458",
    name: "ST Writer document",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x6F, 0x20, 0x52, 0x75, 0x6E, 0x20, 0x52, 0x75, 0x6E, 0x20, 0x53, 0x54,
                    0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x2E, 0x50, 0x52, 0x47, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
