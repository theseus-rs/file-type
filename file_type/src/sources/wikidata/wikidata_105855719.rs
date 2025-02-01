use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855719: FileFormat = FileFormat {
    id: 105_855_719,
    puid: "wikidata/105855719",
    name: "GNU gprof performance data",
    extensions: &["out"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x6D, 0x6F, 0x6E])],
            },
        }],
    }],
    related_formats: &[],
};
