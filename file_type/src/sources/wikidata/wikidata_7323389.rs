use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7323389: FileFormat = FileFormat {
    id: 7_323_389,
    puid: "wikidata/7323389",
    name: "Rich Music Format",
    extensions: &["rmf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x52, 0x45, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
