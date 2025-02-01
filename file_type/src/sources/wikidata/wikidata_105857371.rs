use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857371: FileFormat = FileFormat {
    id: 105_857_371,
    puid: "wikidata/105857371",
    name: "BigJig Jigsaw",
    extensions: &["jg6"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x47, 0x4A, 0x49, 0x47, 0x36, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
