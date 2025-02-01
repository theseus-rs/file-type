use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858970: FileFormat = FileFormat {
    id: 105_858_970,
    puid: "wikidata/105858970",
    name: "Corel Photo Paint bitmap (v9)",
    extensions: &["cpt"],
    media_types: &["image/x-corel-cpt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x50, 0x54, 0x39, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
