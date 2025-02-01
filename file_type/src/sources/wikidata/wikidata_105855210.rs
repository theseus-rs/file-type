use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855210: FileFormat = FileFormat {
    id: 105_855_210,
    puid: "wikidata/105855210",
    name: "FLIP Bitmap",
    extensions: &["fbi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF0, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
