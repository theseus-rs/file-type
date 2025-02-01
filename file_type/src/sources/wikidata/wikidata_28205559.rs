use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205559: FileFormat = FileFormat {
    id: 28_205_559,
    puid: "wikidata/28205559",
    name: "Nokia Operator Logo",
    extensions: &["nol"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x4F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
