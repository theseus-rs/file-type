use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17081599: FileFormat = FileFormat {
    id: 17_081_599,
    puid: "wikidata/17081599",
    name: "Smile",
    extensions: &["sml"],
    media_types: &["application/x-jackson-smile"],
    internal_signatures: &[InternalSignature {
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
