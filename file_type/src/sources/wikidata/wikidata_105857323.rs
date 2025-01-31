use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857323: FileFormat = FileFormat {
    id: 105_857_323,
    puid: "wikidata/105857323",
    name: "Jnes save state",
    extensions: &["jst"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x53, 0x54, 0x60])],
            },
        }],
    }],
    related_formats: &[],
};
