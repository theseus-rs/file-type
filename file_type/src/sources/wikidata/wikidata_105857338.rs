use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857338: FileFormat = FileFormat {
    id: 105_857_338,
    puid: "wikidata/105857338",
    name: "Firefox bookmark (JavaScript Object Notation)",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
