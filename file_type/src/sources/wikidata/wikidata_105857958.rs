use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857958: FileFormat = FileFormat {
    id: 105_857_958,
    puid: "wikidata/105857958",
    name: "Android boot image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4E, 0x44, 0x52, 0x4F, 0x49, 0x44, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
