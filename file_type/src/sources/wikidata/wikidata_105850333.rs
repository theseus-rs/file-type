use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850333: FileFormat = FileFormat {
    id: 105_850_333,
    puid: "wikidata/105850333",
    name: "ApBasic Chain file/module",
    extensions: &["chn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x4D, 0x80, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
