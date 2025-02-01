use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865018: FileFormat = FileFormat {
    id: 105_865_018,
    puid: "wikidata/105865018",
    name: "MegaPaint Pattern",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x50, 0x41, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
