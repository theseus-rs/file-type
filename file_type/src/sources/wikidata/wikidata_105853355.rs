use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853355: FileFormat = FileFormat {
    id: 105_853_355,
    puid: "wikidata/105853355",
    name: "CADVANCE 2D symbol",
    extensions: &["sym"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x3A, 0x32, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
