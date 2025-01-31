use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867236: FileFormat = FileFormat {
    id: 105_867_236,
    puid: "wikidata/105867236",
    name: "JB BAHN layout",
    extensions: &["nt3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
