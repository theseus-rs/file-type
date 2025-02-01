use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2190356: FileFormat = FileFormat {
    id: 2_190_356,
    puid: "wikidata/2190356",
    name: "JPEG-LS",
    extensions: &["jls"],
    media_types: &["image/jls"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xD8, 0xFF, 0xF7, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
