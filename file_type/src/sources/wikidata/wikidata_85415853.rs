use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85415853: FileFormat = FileFormat {
    id: 85_415_853,
    puid: "wikidata/85415853",
    name: "SureThing Project File",
    extensions: &["std"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x56, 0x00, 0xFF, 0x0C, 0x00, 0x12, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
