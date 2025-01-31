use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859806: FileFormat = FileFormat {
    id: 105_859_806,
    puid: "wikidata/105859806",
    name: "ViX images catalog",
    extensions: &["vix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
