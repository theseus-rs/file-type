use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852815: FileFormat = FileFormat {
    id: 105_852_815,
    puid: "wikidata/105852815",
    name: "Swords of the Stars Ship",
    extensions: &["shipsection"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x68, 0x69, 0x70, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
