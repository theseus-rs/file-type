use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850912: FileFormat = FileFormat {
    id: 105_850_912,
    puid: "wikidata/105850912",
    name: "Mario Kart Wii course description",
    extensions: &["kmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4B, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
