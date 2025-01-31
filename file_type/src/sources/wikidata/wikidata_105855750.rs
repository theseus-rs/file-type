use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855750: FileFormat = FileFormat {
    id: 105_855_750,
    puid: "wikidata/105855750",
    name: "Bentley MicroStation CAD drawing (simple)",
    extensions: &["dgn"],
    media_types: &["application/x-bentley-dgn"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x08, 0x09, 0xFE, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
