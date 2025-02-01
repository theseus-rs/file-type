use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853606: FileFormat = FileFormat {
    id: 105_853_606,
    puid: "wikidata/105853606",
    name: "Zoot information processor database",
    extensions: &["zot"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x4F, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
