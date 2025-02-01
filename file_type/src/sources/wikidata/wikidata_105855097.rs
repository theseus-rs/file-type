use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855097: FileFormat = FileFormat {
    id: 105_855_097,
    puid: "wikidata/105855097",
    name: "777 compressed archive",
    extensions: &["777"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x37, 0x37, 0x37])],
            },
        }],
    }],
    related_formats: &[],
};
