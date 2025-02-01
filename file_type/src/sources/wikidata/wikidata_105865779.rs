use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865779: FileFormat = FileFormat {
    id: 105_865_779,
    puid: "wikidata/105865779",
    name: "Hugin Project",
    extensions: &["pto"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
