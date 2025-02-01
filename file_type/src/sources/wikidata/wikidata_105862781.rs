use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862781: FileFormat = FileFormat {
    id: 105_862_781,
    puid: "wikidata/105862781",
    name: "COMSOL Multiphysics mesh (txt)",
    extensions: &["mphtxt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
