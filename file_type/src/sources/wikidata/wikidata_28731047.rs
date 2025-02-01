use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28731047: FileFormat = FileFormat {
    id: 28_731_047,
    puid: "wikidata/28731047",
    name: "Dyalog APL workspace",
    extensions: &["dws"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
