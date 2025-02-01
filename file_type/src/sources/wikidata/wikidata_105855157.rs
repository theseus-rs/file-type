use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855157: FileFormat = FileFormat {
    id: 105_855_157,
    puid: "wikidata/105855157",
    name: "ABBYY eForm Filler data",
    extensions: &["ffdata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
