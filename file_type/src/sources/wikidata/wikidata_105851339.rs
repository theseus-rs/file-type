use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851339: FileFormat = FileFormat {
    id: 105_851_339,
    puid: "wikidata/105851339",
    name: "Wintertree dictionary",
    extensions: &["tlx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x4C, 0x49, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
