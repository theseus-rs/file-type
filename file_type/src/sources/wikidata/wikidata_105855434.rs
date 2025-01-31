use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855434: FileFormat = FileFormat {
    id: 105_855_434,
    puid: "wikidata/105855434",
    name: "Facet file",
    extensions: &["facet"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x41, 0x43, 0x45, 0x54, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x46, 0x52,
                    0x4F, 0x4D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
