use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853970: FileFormat = FileFormat {
    id: 105_853_970,
    puid: "wikidata/105853970",
    name: "Aperture Album",
    extensions: &["apalbum"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
