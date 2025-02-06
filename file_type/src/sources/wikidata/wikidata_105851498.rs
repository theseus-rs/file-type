use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851498: FileFormat = FileFormat {
    id: 105_851_498,
    source_type: SourceType::Wikidata,
    name: "Spybot Search'n'Destroy process data",
    extensions: &["tnfo"],
    media_types: &["text/plain"],
    signatures: &[Signature {
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
