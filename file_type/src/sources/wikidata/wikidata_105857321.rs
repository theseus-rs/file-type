use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857321: FileFormat = FileFormat {
    id: 105_857_321,
    puid: "wikidata/105857321",
    name: "I.E.S. HyperText",
    extensions: &["hyp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x48, 0x04, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
