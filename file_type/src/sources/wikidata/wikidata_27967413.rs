use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967413: FileFormat = FileFormat {
    id: 27_967_413,
    puid: "wikidata/27967413",
    name: "DOSBox Raw OPL",
    extensions: &["dro"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x42, 0x52, 0x41, 0x57, 0x4F, 0x50, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
