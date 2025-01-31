use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206193: FileFormat = FileFormat {
    id: 28_206_193,
    puid: "wikidata/28206193",
    name: "GoDot 4-bit image",
    extensions: &["4bt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x44, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
