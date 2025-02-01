use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851433: FileFormat = FileFormat {
    id: 105_851_433,
    puid: "wikidata/105851433",
    name: "TuneUp Styler Logo Animation",
    extensions: &["tla"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
