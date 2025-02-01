use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850233: FileFormat = FileFormat {
    id: 105_850_233,
    puid: "wikidata/105850233",
    name: "The Virtual ColecoVision Save Game",
    extensions: &["csg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x47, 0x0A, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
