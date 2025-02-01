use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865995: FileFormat = FileFormat {
    id: 105_865_995,
    puid: "wikidata/105865995",
    name: "Commodore 128 BASIC V7.0 program",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
