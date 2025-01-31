use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851498: FileFormat = FileFormat {
    id: 105_851_498,
    puid: "wikidata/105851498",
    name: "Spybot Search'n'Destroy process data",
    extensions: &["tnfo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x5D, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
