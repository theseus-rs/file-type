use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857110: FileFormat = FileFormat {
    id: 105_857_110,
    puid: "wikidata/105857110",
    name: "gBurner Image",
    extensions: &["gbi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x42, 0x49, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
