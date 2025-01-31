use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853657: FileFormat = FileFormat {
    id: 105_853_657,
    puid: "wikidata/105853657",
    name: "AIMP Skin (v2)",
    extensions: &["acs2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
