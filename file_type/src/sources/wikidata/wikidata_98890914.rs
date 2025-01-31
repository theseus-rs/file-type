use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_98890914: FileFormat = FileFormat {
    id: 98_890_914,
    puid: "wikidata/98890914",
    name: "Windows Calendar",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xB5, 0xA2, 0xB0, 0xB3, 0xB3, 0xB0, 0xA2, 0xB5,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
