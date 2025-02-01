use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855379: FileFormat = FileFormat {
    id: 105_855_379,
    puid: "wikidata/105855379",
    name: "Reunion Family tree (v8)",
    extensions: &["familyfile"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x44, 0x53, 0x33, 0x52, 0x7E, 0x55, 0x38,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
