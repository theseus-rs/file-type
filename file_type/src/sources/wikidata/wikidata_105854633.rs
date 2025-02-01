use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854633: FileFormat = FileFormat {
    id: 105_854_633,
    puid: "wikidata/105854633",
    name: "BCM compressed archive",
    extensions: &["bcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0x4D, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
