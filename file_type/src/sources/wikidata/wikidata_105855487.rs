use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855487: FileFormat = FileFormat {
    id: 105_855_487,
    puid: "wikidata/105855487",
    name: "Colton Software Fireworkz document",
    extensions: &["fwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
