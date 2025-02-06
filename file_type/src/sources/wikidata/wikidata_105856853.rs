use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856853: FileFormat = FileFormat {
    id: 105_856_853,
    source_type: SourceType::Wikidata,
    name: "Google Maps API data",
    extensions: &["getviewportinfo"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5F, 0x78, 0x64, 0x63, 0x5F, 0x2E, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
