use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207510: FileFormat = FileFormat {
    id: 28_207_510,
    puid: "wikidata/28207510",
    name: "Winzle Puzzle",
    extensions: &["wzl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
