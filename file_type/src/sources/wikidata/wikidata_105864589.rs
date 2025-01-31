use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864589: FileFormat = FileFormat {
    id: 105_864_589,
    puid: "wikidata/105864589",
    name: "CorelDRAW Pattern (v2.0)",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x4C, 0x6C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
