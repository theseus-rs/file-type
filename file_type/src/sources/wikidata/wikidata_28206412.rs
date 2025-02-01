use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206412: FileFormat = FileFormat {
    id: 28_206_412,
    puid: "wikidata/28206412",
    name: "Jovian Logic VI",
    extensions: &["vi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
