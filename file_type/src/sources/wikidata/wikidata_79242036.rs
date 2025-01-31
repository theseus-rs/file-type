use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79242036: FileFormat = FileFormat {
    id: 79_242_036,
    puid: "wikidata/79242036",
    name: "American College of Radiology file",
    extensions: &["acr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x63, 0x63, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
