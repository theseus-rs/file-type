use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967294: FileFormat = FileFormat {
    id: 27_967_294,
    puid: "wikidata/27967294",
    name: "Musink project",
    extensions: &["musink"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
