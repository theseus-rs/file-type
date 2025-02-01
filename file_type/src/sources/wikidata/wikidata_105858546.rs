use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858546: FileFormat = FileFormat {
    id: 105_858_546,
    puid: "wikidata/105858546",
    name: "Blizzard Picture (type 2)",
    extensions: &["blp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4C, 0x50, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
