use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17081599: FileFormat = FileFormat {
    id: 17_081_599,
    source_type: SourceType::Wikidata,
    name: "Smile",
    extensions: &["sml"],
    media_types: &["application/x-jackson-smile"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x29, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
