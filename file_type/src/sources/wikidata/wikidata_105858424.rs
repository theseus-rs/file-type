use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858424: FileFormat = FileFormat {
    id: 105_858_424,
    puid: "wikidata/105858424",
    name: "Solace ENTER hex format (abbreviated)",
    extensions: &["ent"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
