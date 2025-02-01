use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2357210: FileFormat = FileFormat {
    id: 2_357_210,
    puid: "wikidata/2357210",
    name: "Structured Fax File",
    extensions: &["sff"],
    media_types: &["image/x-sff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x66, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
