use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29151555: FileFormat = FileFormat {
    id: 29_151_555,
    puid: "wikidata/29151555",
    name: "RAGE Package Format",
    extensions: &["rpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
