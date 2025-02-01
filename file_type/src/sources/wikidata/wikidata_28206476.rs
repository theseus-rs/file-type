use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206476: FileFormat = FileFormat {
    id: 28_206_476,
    puid: "wikidata/28206476",
    name: "Kolor Raw",
    extensions: &["kro"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x52, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
