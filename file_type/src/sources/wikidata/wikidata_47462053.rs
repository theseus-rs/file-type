use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47462053: FileFormat = FileFormat {
    id: 47_462_053,
    puid: "wikidata/47462053",
    name: "Siegfried Signature File",
    extensions: &["sig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x66, 0x00, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
