use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049414: FileFormat = FileFormat {
    id: 28_049_414,
    puid: "wikidata/28049414",
    name: "DEGAS image, medium resolution",
    extensions: &["PI2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
