use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857502: FileFormat = FileFormat {
    id: 105_857_502,
    puid: "wikidata/105857502",
    name: "AIMutation skin",
    extensions: &["545"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04, 0x14])],
            },
        }],
    }],
    related_formats: &[],
};
