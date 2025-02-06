use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856945: FileFormat = FileFormat {
    id: 105_856_945,
    source_type: SourceType::Wikidata,
    name: "GEOS Geode Parameters",
    extensions: &["gp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
