use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865720: FileFormat = FileFormat {
    id: 105_865_720,
    puid: "wikidata/105865720",
    name: "Commodore Plus/4 BASIC V3.5 program",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
