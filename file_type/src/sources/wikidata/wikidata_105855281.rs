use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855281: FileFormat = FileFormat {
    id: 105_855_281,
    puid: "wikidata/105855281",
    name: "Farandole F3R blocked linear module format",
    extensions: &["f3r"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x33, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
