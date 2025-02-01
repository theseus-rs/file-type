use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858493: FileFormat = FileFormat {
    id: 105_858_493,
    puid: "wikidata/105858493",
    name: "ZZ ROUGH bitmap",
    extensions: &["rgh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x63, 0x29, 0x46, 0x2E, 0x4D, 0x41, 0x52, 0x43, 0x48, 0x41, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
