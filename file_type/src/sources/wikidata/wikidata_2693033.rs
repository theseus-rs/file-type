use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2693033: FileFormat = FileFormat {
    id: 2_693_033,
    puid: "wikidata/2693033",
    name: "ARJ",
    extensions: &["arj"],
    media_types: &["application/arj"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60, 0xEA])],
            },
        }],
    }],
    related_formats: &[],
};
