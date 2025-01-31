use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854628: FileFormat = FileFormat {
    id: 105_854_628,
    puid: "wikidata/105854628",
    name: "Mii Maker data archive",
    extensions: &["arc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x52, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
