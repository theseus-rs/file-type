use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867559: FileFormat = FileFormat {
    id: 105_867_559,
    puid: "wikidata/105867559",
    name: "NVIDIA Scene Graph binary",
    extensions: &["nbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x4E, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
