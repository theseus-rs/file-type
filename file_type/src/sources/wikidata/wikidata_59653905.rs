use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59653905: FileFormat = FileFormat {
    id: 59_653_905,
    puid: "wikidata/59653905",
    name: "Maya ASCII File Format",
    extensions: &["ma"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x4D, 0x61, 0x79, 0x61, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
