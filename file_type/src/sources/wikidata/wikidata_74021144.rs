use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74021144: FileFormat = FileFormat {
    id: 74_021_144,
    source_type: SourceType::Wikidata,
    name: "PICS Rating System",
    extensions: &["rat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28])],
            },
        }],
    }],
    related_formats: &[],
};
