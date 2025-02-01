use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850541: FileFormat = FileFormat {
    id: 105_850_541,
    puid: "wikidata/105850541",
    name: "Forte Agent Charmap",
    extensions: &["cod"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x61, 0x6D, 0x65, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
