use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857273: FileFormat = FileFormat {
    id: 105_857_273,
    puid: "wikidata/105857273",
    name: "Quick Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x8B, 0x92])],
            },
        }],
    }],
    related_formats: &[],
};
