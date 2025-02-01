use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855858: FileFormat = FileFormat {
    id: 105_855_858,
    puid: "wikidata/105855858",
    name: "Dawn file format",
    extensions: &["dwn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x41, 0x57, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
