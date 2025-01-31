use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1036298: FileFormat = FileFormat {
    id: 1_036_298,
    puid: "wikidata/1036298",
    name: "HighMAT",
    extensions: &["hmt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4D, 0x54, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
