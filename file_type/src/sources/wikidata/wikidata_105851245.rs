use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851245: FileFormat = FileFormat {
    id: 105_851_245,
    puid: "wikidata/105851245",
    name: "TValue project",
    extensions: &["tv4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC3, 0x54, 0x56, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
