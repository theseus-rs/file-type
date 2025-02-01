use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858476: FileFormat = FileFormat {
    id: 105_858_476,
    puid: "wikidata/105858476",
    name: "Open Publication Structure eBook",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
