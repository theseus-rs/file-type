use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854374: FileFormat = FileFormat {
    id: 105_854_374,
    puid: "wikidata/105854374",
    name: "Aladdin 4D ATList",
    extensions: &["atl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x41, 0x54, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
