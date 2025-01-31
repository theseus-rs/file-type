use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857919: FileFormat = FileFormat {
    id: 105_857_919,
    puid: "wikidata/105857919",
    name: "Img3 enctrypted signed container",
    extensions: &["img3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x67, 0x6D, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
