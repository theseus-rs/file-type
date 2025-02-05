use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856320: FileFormat = FileFormat {
    id: 105_856_320,
    source_type: SourceType::Wikidata,
    name: "Dagesh document",
    extensions: &["dgs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x56, 0x45, 0x52, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
