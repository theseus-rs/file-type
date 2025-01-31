use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853314: FileFormat = FileFormat {
    id: 105_853_314,
    puid: "wikidata/105853314",
    name: "Brother/Babylock/Bernina Home Embroidery Format",
    extensions: &["pes"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x50, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
