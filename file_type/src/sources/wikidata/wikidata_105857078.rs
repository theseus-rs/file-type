use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857078: FileFormat = FileFormat {
    id: 105_857_078,
    puid: "wikidata/105857078",
    name: "Windows Program Manager Group",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x43, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
