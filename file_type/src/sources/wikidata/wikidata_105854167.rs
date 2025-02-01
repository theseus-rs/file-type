use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854167: FileFormat = FileFormat {
    id: 105_854_167,
    puid: "wikidata/105854167",
    name: "Digital Speech Standard audio (v3)",
    extensions: &["dss"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x64, 0x73, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
