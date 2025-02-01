use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855257: FileFormat = FileFormat {
    id: 105_855_257,
    puid: "wikidata/105855257",
    name: "Forgotten Worlds custom music format",
    extensions: &["fw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x57, 0x4D, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
