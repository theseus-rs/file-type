use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865968: FileFormat = FileFormat {
    id: 105_865_968,
    puid: "wikidata/105865968",
    name: "Paragon Backup Format image",
    extensions: &["pbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x6D, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
