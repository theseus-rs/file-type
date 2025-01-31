use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5183527: FileFormat = FileFormat {
    id: 5_183_527,
    puid: "wikidata/5183527",
    name: "Creative Music Format",
    extensions: &["cmf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x54, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
