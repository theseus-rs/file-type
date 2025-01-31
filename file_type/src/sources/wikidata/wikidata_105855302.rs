use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855302: FileFormat = FileFormat {
    id: 105_855_302,
    puid: "wikidata/105855302",
    name: "Far Cry 3 map",
    extensions: &["fc3map"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x13, 0x00, 0x00, 0x00, 0x6B, 0x0A, 0xFD, 0xD2,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
