use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853578: FileFormat = FileFormat {
    id: 105_853_578,
    puid: "wikidata/105853578",
    name: "Pack compressed data (old)",
    extensions: &["z"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};
