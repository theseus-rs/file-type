use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854296: FileFormat = FileFormat {
    id: 105_854_296,
    puid: "wikidata/105854296",
    name: "MPEG-2 LC-AAC Audio",
    extensions: &["aac"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xF9])],
            },
        }],
    }],
    related_formats: &[],
};
