use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4039139: FileFormat = FileFormat {
    id: 4_039_139,
    puid: "wikidata/4039139",
    name: "Ghost image",
    extensions: &["gho"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xEF])],
            },
        }],
    }],
    related_formats: &[],
};
