use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850639: FileFormat = FileFormat {
    id: 105_850_639,
    puid: "wikidata/105850639",
    name: "Consolidated Laser Ranging Prediction Format",
    extensions: &["cpf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x31, 0x20, 0x43, 0x50, 0x46, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
