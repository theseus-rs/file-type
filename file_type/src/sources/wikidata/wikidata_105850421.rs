use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850421: FileFormat = FileFormat {
    id: 105_850_421,
    puid: "wikidata/105850421",
    name: "Cal3D morphanimation File",
    extensions: &["cpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x50, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
