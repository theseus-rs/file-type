use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855035: FileFormat = FileFormat {
    id: 105_855_035,
    puid: "wikidata/105855035",
    name: "Windows Policy Administrative Template",
    extensions: &["adm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4C, 0x41, 0x53, 0x53, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
