use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967387: FileFormat = FileFormat {
    id: 27_967_387,
    puid: "wikidata/27967387",
    name: "AdLib instrument bank",
    extensions: &["bnk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x49, 0x42, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
