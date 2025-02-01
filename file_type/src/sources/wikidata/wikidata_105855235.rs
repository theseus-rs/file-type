use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855235: FileFormat = FileFormat {
    id: 105_855_235,
    puid: "wikidata/105855235",
    name: "FCE Ultra FC0 savestate",
    extensions: &["fc0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
