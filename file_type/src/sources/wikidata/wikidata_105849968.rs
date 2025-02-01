use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849968: FileFormat = FileFormat {
    id: 105_849_968,
    puid: "wikidata/105849968",
    name: "Turbo Pascal 2.0 Chain module",
    extensions: &["chn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE8, 0x03, 0xE3])],
            },
        }],
    }],
    related_formats: &[],
};
