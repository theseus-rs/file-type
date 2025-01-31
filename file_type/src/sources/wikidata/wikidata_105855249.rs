use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855249: FileFormat = FileFormat {
    id: 105_855_249,
    puid: "wikidata/105855249",
    name: "Origin Fitting Function Definition File",
    extensions: &["fdf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
