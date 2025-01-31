use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853599: FileFormat = FileFormat {
    id: 105_853_599,
    puid: "wikidata/105853599",
    name: "Pack compressed data",
    extensions: &["z"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x1E])],
            },
        }],
    }],
    related_formats: &[],
};
